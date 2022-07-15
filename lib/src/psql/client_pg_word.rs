use uuid::Uuid;

use crate::psql::sqlbuilder::{Join, SqlBuilder};
use crate::{PaginationOptions, Word, WordQueryOptions};

use super::models::{PgTranslation, PgTranslationEntry, PgWord, PgWordType};
use super::sqlbuilder::{Updatable, Value};
use super::{PgContext, PgDBClient};

impl PgDBClient {
    pub fn handle_insert_word(
        &self,
        ctx: &mut PgContext,
        new_word: &mut crate::Word,
    ) -> Result<uuid::Uuid, ()> {
        let wdbm: PgWord = new_word.into();

        let (query, params) = SqlBuilder::insert("words")
            // .values(&wdbm)
            // .returning("id")
            .build();
        match ctx.conn.query(&query, params) {
            Ok(rows) => {
                let id: Uuid = rows.get(0).unwrap().get(0);
                Ok(id)
            }
            Err(_) => Err(()),
        }
    }

    pub fn handle_delete_word(&self, ctx: &mut PgContext, word_id: uuid::Uuid) -> Result<(), ()> {
        // Delete both the word and its related translations
        let (query, params) = SqlBuilder::delete("words")
            .filter("id = ?")
            .bind(Box::new(word_id))
            .build();
        ctx.conn
            .execute(&query, &params)
            .map(|_| ())
            .map_err(|err| {
                println!("{:?}", err);
                ()
            })
    }

    pub fn handle_update_word(
        &self,
        ctx: &mut PgContext,
        update_options: &crate::WordUpdateOptions,
    ) -> Result<Word, ()> {
        let (query, params) = SqlBuilder::update("words")
            .filter("id = ?")
            .bind(Box::new(update_options.id))
            .set(update_options)
            .returning("*")
            .build();
        match ctx.conn.query(&query, params) {
            Ok(rows) => {
                let word: PgWord = rows.get(0).unwrap().into();
                Ok((&word).into())
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_query_words(
        &self,
        ctx: &mut PgContext,
        query_options: &WordQueryOptions,
        pagination_options: &PaginationOptions,
    ) -> Result<crate::PageResult<Word>, ()> {
        let mut qb = SqlBuilder::select("words")
            .join(Join::CrossJoin("unnest(translations) as t".to_string()));

        if let Some(q) = &query_options.query {
            qb = qb
                .filter("t.value SIMILAR TO '%?%'")
                .bind(Box::new(&q))
                .filter("word SIMILAR TO '%?%'")
                .bind(Box::new(&q));
        }

        if let Some(kind) = &query_options.kind {
            qb = qb.filter("kind = ?").bind(Box::new(&kind.to_string()));
        }

        if let Some(tags) = &query_options.tags {
            qb = qb.filter("tags @> ?").bind(Box::new(&tags.clone()));
        }

        if let Some(word) = &query_options.word {
            qb = qb.filter("word = ?").bind(Box::new(&word.clone()));
        }

        let (query, params) = qb.build();
        match ctx.conn.query(&query, params) {
            Ok(res) => {
                let words = res
                    .into_iter()
                    .map(|ref r| r.into())
                    .collect::<Vec<PgWord>>()
                    .into_iter()
                    .map(|ref word| word.into())
                    .collect::<Vec<Word>>();
                Ok(crate::PageResult {
                    total: words.len(),
                    page: todo!(),
                    count: todo!(),
                    results: words,
                })
            }
            Err(err) => {
                println!("{:?}", err);
                return Err(());
            }
        }

        // if let Some(q) = &query_options.query {
        //     query.bind::<Text, _>(q);
        // }

        // if let Some(kind) = &query_options.kind {
        //     query.bind::<Text, _>(kind.to_string());
        // }

        // if let Some(tags) = &query_options.tags {
        //     query.bind::<Array<Text>, _>(tags.clone());
        // }

        // if let Some(word) = &query_options.word {
        //     query.bind::<Text, _>(word.clone());
        // }

        // let mut query = words::table
        //     // .inner_join(sql("unnest(words.translations)"))
        // //     .into_boxed::<Pg>();

        // if let Some(query) = query_options.query.clone() {
        //     query = query.filter(
        //         sql("word SIMILAR TO '%{}%'")
        //             .bind::<Text, _>(&query)
        //             .into_sql(),
        //     );
        // }

        // if let Some(word) = query_options.word {
        //     query = query.filter(words::word.eq(word));
        // }

        // if let Some(word_type) = query_options.kind {
        //     let a = words::kind.eq(PgWordType::from(word_type));
        //     query = query.filter(a);
        // }

        // if let Some(tags) = query_options.tags {
        //     query = query.filter(words::tags.overlaps_with(tags));
        // }

        // query
        //     .limit(pagination_options.limit)
        //     .offset(pagination_options.offset)
    }

    pub fn handle_get_words(
        &self,
        ctx: &mut PgContext,
        ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<Word>, ()> {
        let (query, params) = SqlBuilder::select("words")
            .filter("id IN ?")
            .bind(Box::new(ids))
            .build();
        match ctx.conn.query(&query, params) {
            Ok(res) => {
                let words = res
                    .into_iter()
                    .map(|ref r| r.into())
                    .collect::<Vec<PgWord>>()
                    .into_iter()
                    .map(|ref word| word.into())
                    .collect::<Vec<Word>>();
                Ok(words)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_get_random_words(
        &self,
        ctx: &mut PgContext,
        count: i32,
    ) -> Result<Vec<Word>, ()> {
        let (query, params) = SqlBuilder::select("words")
            .order_by("RANDOM()")
            .limit(count as i64)
            .build();
        match ctx.conn.query(&query, params) {
            Ok(res) => {
                let words = res
                    .into_iter()
                    .map(|ref r| r.into())
                    .collect::<Vec<PgWord>>()
                    .into_iter()
                    .map(|ref word| word.into())
                    .collect::<Vec<Word>>();
                Ok(words)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
}

impl Updatable for crate::WordUpdateOptions {
    fn columns(&self) -> Vec<(&str, Value)> {
        let mut columns: Vec<(&str, Value)> = Vec::new();
        if let Some(ref word) = self.word {
            columns.push(("word", Box::new(word)));
        }
        if let Some(kind) = self.kind {
            let pg_kind = PgWordType::from(kind);
            columns.push(("kind", Box::new(pg_kind)));
        }
        if let Some(ref tags) = self.tags {
            columns.push(("tags", Box::new(tags)));
        }
        if let Some(ref translations) = self.translations {
            let pg_translations: Vec<PgTranslation> =
                translations.iter().map(|t| t.into()).collect();
            columns.push(("translations", Box::new(pg_translations)));
        }
        columns
    }
}

// impl AsChangeset for &crate::WordUpdateOptions {
//     type Target = words::table;
//     type Changeset = (
//         Option<Eq<words::word, Bound<Text, String>>>,
//         Option<Eq<words::kind, Bound<PgWordTypeMapping, PgWordType>>>,
//         Option<Eq<words::tags, Vec<String>>>,
//         Option<Eq<words::translations, Bound<Array<PgTranslationEntry>, Vec<PgTranslation>>>>,
//         Option<Eq<words::updated_at, NaiveDateTime>>,
//     );

//     fn as_changeset(self) -> Self::Changeset {
//         (
//             self.word.clone().map(|f| words::word.eq(f)),
//             self.kind.map(|f| words::kind.eq::<PgWordType>(f.into())),
//             self.tags.clone().map(|f| words::tags.eq(f)),
//             self.translations.clone().map(|t| {
//                 words::translations.eq::<Vec<PgTranslation>>(t.iter().map(|t| t.into()).collect())
//             }),
//             Some(words::updated_at.eq(Utc::now().naive_utc())),
//         )
//     }
// }
