use std::fmt::Display;

use r2d2_postgres::postgres::types::ToSql;

enum QueryType {
    SELECT,
    INSERT,
    UPDATE,
    DELETE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Join {
    Single(String),
    CrossJoin(String),
    InnerJoin(String, String),
    LeftJoin(String, String),
}

impl<'a> Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Join::Single(table) => write!(f, "{}", table),
            Join::CrossJoin(table) => write!(f, ", {}", table),
            Join::InnerJoin(table, condition) => {
                write!(f, "INNER JOIN {} ON ({})", table, condition)
            }
            Join::LeftJoin(table, condition) => {
                write!(f, "LEFT JOIN {} ON ({})", table, condition)
            }
        }
    }
}

type RawValue = dyn r2d2_postgres::postgres::types::ToSql + Sync;
pub type Value = Box<RawValue>;

pub trait Insertable {
    fn columns(&self) -> Vec<&str>;
    fn values(&self) -> Vec<Value>;
}

pub trait Updatable {
    fn columns(&self) -> Vec<(&str, Value)>;
}

pub struct SqlBuilder {
    query_type: QueryType,

    // For select queries
    projections: Vec<String>, // SELECT
    tables: Vec<Join>,        // FROM
    orders: Vec<String>,      // ORDER BY
    limit: Option<i64>,       // LIMIT

    // For inserts and updates
    columns: Option<Vec<String>>,

    // All
    parameters: Vec<Value>,
    conditions: Vec<String>, // WHERE
    returning: Option<String>,
}

impl SqlBuilder {
    fn new(table: &str, query_type: QueryType) -> Self {
        SqlBuilder {
            query_type,
            projections: vec![],
            tables: vec![Join::Single(table.to_string())],
            orders: vec![],
            limit: None,

            conditions: vec![],
            parameters: vec![],

            columns: None,

            returning: None,
        }
    }

    pub fn select(table: &str) -> Self {
        SqlBuilder::new(table, QueryType::SELECT)
    }

    pub fn insert(table: &str) -> Self {
        SqlBuilder::new(table, QueryType::INSERT)
    }

    pub fn update(table: &str) -> Self {
        SqlBuilder::new(table, QueryType::UPDATE)
    }

    pub fn delete(table: &str) -> Self {
        SqlBuilder::new(table, QueryType::DELETE)
    }

    pub fn join(&mut self, table: Join) -> &mut Self {
        self.tables.push(table.clone());
        self
    }

    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.conditions.push(filter.to_string());
        self
    }

    pub fn bind(&mut self, value: Value) -> &mut Self {
        self.parameters.push(value);
        self
    }

    pub fn column(&mut self, field: &str) -> &mut Self {
        self.projections.push(field.to_string());
        self
    }

    // add is for inserts
    pub fn values(&mut self, row: &impl Insertable) -> &mut Self {
        if self.columns.is_none() {
            self.columns = Some(row.columns().iter().map(|c| c.to_string()).collect());
        }

        self.parameters.extend(row.values());
        self
    }

    // set_field is for updates
    pub fn set_field(&mut self, field: &str, value: Value) -> &mut Self {
        if self.columns.is_none() {
            self.columns = Some(vec![field.to_string()]);
        } else {
            self.columns.as_mut().unwrap().push(field.to_string());
        }

        self.parameters.push(value);
        self
    }

    // set is for updates
    pub fn set(&mut self, row: &impl Updatable) -> &mut Self {
        if self.columns.is_none() {
            self.columns = Some(row.columns().iter().map(|c| c.0.to_string()).collect());
        }

        self.parameters.extend(row.columns().iter().map(|c| c.1));

        self
    }

    pub fn order_by(&mut self, field: &str) -> &mut Self {
        self.orders.push(field.to_string());
        self
    }

    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn returning(&mut self, field: &str) -> &mut Self {
        self.returning = Some(field.to_string());
        self
    }

    pub fn build(&self) -> (String, &[&(dyn ToSql + Sync)]) {
        let mut query: Vec<String> = Vec::new();

        match self.query_type {
            QueryType::SELECT => {
                query.push("SELECT".to_string());
                query.push(self.projections.join(", "));
                query.push("FROM".to_string());
                query.push(
                    self.tables
                        .iter()
                        .map(|t| match t {
                            Join::Single(table) => table.to_string(),
                            Join::CrossJoin(table) => format!(", {}", table),
                            Join::InnerJoin(table, condition) => {
                                format!(" INNER JOIN {} ON ({})", table, condition)
                            }
                            Join::LeftJoin(table, condition) => {
                                format!(" LEFT JOIN {} ON ({})", table, condition)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(""),
                );
                self.build_where(&mut query);
                self.build_order_by(&mut query);
                self.build_limit(&mut query);
            }
            QueryType::INSERT => {
                query.push("INSERT INTO".to_string());
                query.push(self.tables[0].to_string());

                let columns = self.columns.as_ref();

                if columns.is_some() && columns.unwrap().len() > 0 {
                    query.push("(".to_string());
                    query.push(self.columns.as_ref().unwrap().join(", "));
                    query.push(")".to_string());
                }

                if self.parameters.len() > 0 && columns.unwrap().len() > 0 {
                    query.push("VALUES".to_string());

                    let num_rows = self.parameters.len() / columns.unwrap().len();
                    query.push(
                        self.parameters
                            .chunks(num_rows)
                            .map(|_| {
                                columns
                                    .iter()
                                    .enumerate()
                                    .map(|(k, _)| String::from("?"))
                                    .collect::<Vec<String>>()
                                    .join(", ")
                            })
                            .collect::<Vec<String>>()
                            .join(", "),
                    );
                }
                self.build_returning(&mut query);
            }
            QueryType::UPDATE => {
                query.push("UPDATE".to_string());
                query.push(self.tables[0].to_string());

                if self.columns.is_some() && self.columns.as_ref().unwrap().len() > 0 {
                    query.push("SET".to_string());
                    query.push(
                        self.columns
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|c| format!("{} = ?", c))
                            .collect::<Vec<String>>()
                            .join(", "),
                    );
                }
                self.build_where(&mut query);
                self.build_returning(&mut query);
            }
            QueryType::DELETE => {
                query.push("DELETE FROM".to_string());
                query.push(self.tables[0].to_string());
                self.build_where(&mut query);
                self.build_returning(&mut query);
            }
        }

        let mut output = format!("{};", query.join(" "));

        let mut n = 1;
        while let Some(index) = output.find("?") {
            output = output.replace("?", &format!("${}", n));
            n += 1;
        }
        (
            output,
            self.parameters
                .iter()
                .map(|f| f.as_ref())
                .collect::<Vec<&RawValue>>()
                .as_slice(),
        )
    }

    fn build_where(&self, query: &mut Vec<String>) {
        if !self.conditions.is_empty() {
            query.push("WHERE".to_string());
            query.push(self.conditions.join(" AND "));
        }
    }

    fn build_returning(&self, query: &mut Vec<String>) {
        if let Some(returning) = &self.returning {
            query.push("RETURNING".to_string());
            query.push(returning.to_string());
        }
    }

    fn build_order_by(&self, query: &mut Vec<String>) {
        if !self.orders.is_empty() {
            query.push("ORDER BY".to_string());
            query.push(self.orders.join(", "));
        }
    }

    fn build_limit(&self, query: &mut Vec<String>) {
        if let Some(limit) = &self.limit {
            query.push("LIMIT".to_string());
            query.push(limit.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::psql::sqlbuilder::Join;

    use super::SqlBuilder;

    #[test]
    fn build_select_query() {
        let (sql, params) = SqlBuilder::select("words")
            .join(Join::CrossJoin("unnest(translations) as t".to_string()))
            .filter("t.value ILIKE '%?%'")
            .bind(Box::new(&"share"))
            .column("*")
            .build();
        assert_eq!(
            sql,
            "SELECT * FROM words, unnest(translations) as t WHERE t.value ILIKE '%$1%';"
        );
    }
}
