package pg

import (
	"strconv"
	"strings"
)

type joinType string

const (
	joinTypeSingle joinType = "single"
	joinTypeLeft   joinType = "left"
	joinTypeRight  joinType = "right"
	joinTypeCross  joinType = "cross"
)

type join struct {
	table     string
	joinType  joinType
	condition *string
}

type operation string

const (
	opAND operation = "AND"
	opOR  operation = "OR"
)

type condition struct {
	condition  string
	op         operation
	conditions []condition
	parameters []interface{}
}

func (q *condition) Where(where string, parameters ...interface{}) *condition {
	if q.conditions == nil {
		q.conditions = []condition{}
	}
	q.conditions = append(q.conditions, condition{where, opAND, nil, parameters})
	return q
}

func (q *condition) WhereOr(where string, parameters ...interface{}) *condition {
	if q.conditions == nil {
		q.conditions = []condition{}
	}
	q.conditions = append(q.conditions, condition{where, opOR, nil, parameters})
	return q
}

func (c *condition) String() string {
	if len(c.conditions) > 0 {
		conditions := strings.Builder{}
		conditions.WriteString("(")
		for i, condition := range c.conditions {
			if i > 0 {
				conditions.WriteString(" " + string(condition.op) + " ")
			}
			conditions.WriteString(condition.String())
		}
		conditions.WriteString(")")
		return conditions.String()
	}
	return c.condition
}

func (c *condition) Params() []interface{} {
	if len(c.conditions) > 0 {
		params := []interface{}{}
		for _, condition := range c.conditions {
			params = append(params, condition.Params()...)
		}
		return params
	}
	return c.parameters
}

type Query struct {
	columns    []string
	tables     []join
	conditions []condition

	orderBy []string
	offset  *int
	limit   *int
}

func NewQuery(table string) Query {
	return Query{
		columns:    []string{},
		conditions: []condition{},
		tables:     []join{{table, joinTypeSingle, nil}},
	}
}

func (q Query) Column(column string) Query {
	q.columns = append(q.columns, column)
	return q
}

func (q Query) Count(column string) Query {
	q.columns = []string{"COUNT(" + column + ")"}
	return q
}

func (q Query) LeftJoin(table string, condition string) Query {
	q.tables = append(q.tables, join{table, joinTypeLeft, &condition})
	return q
}

func (q Query) CrossJoin(table string) Query {
	q.tables = append(q.tables, join{table, joinTypeCross, nil})
	return q
}

func (q Query) Where(where string, parameters ...interface{}) Query {
	q.conditions = append(q.conditions, condition{where, opAND, nil, parameters})
	return q
}

func (q Query) WhereOr(where string, parameters ...interface{}) Query {
	q.conditions = append(q.conditions, condition{where, opOR, nil, parameters})
	return q
}

func (q Query) WhereGroup(fn func(c *condition)) Query {
	cond := condition{"", opAND, nil, nil}
	fn(&cond)
	if len(cond.conditions) > 0 {
		q.conditions = append(q.conditions, cond)
	}
	return q
}

func (q Query) OrderBy(orderBy string) Query {
	q.orderBy = append(q.orderBy, orderBy)
	return q
}

func (q Query) Offset(offset int) Query {
	q.offset = &offset
	return q
}

func (q Query) Limit(limit int) Query {
	q.limit = &limit
	return q
}

func (q Query) Build() (string, []interface{}) {

	query := strings.Builder{}
	params := []interface{}{}
	query.WriteString("SELECT ")
	if len(q.columns) > 0 {
		for i, column := range q.columns {
			if i > 0 {
				query.WriteString(", ")
			}
			query.WriteString(column)
		}
	} else {
		query.WriteString("*")
	}
	query.WriteString(" FROM ")
	for _, table := range q.tables {
		if table.joinType == joinTypeLeft {
			query.WriteString(" LEFT JOIN ")
		} else if table.joinType == joinTypeRight {
			query.WriteString(" RIGHT JOIN ")
		} else if table.joinType == joinTypeCross {
			query.WriteString(", ")
		}
		query.WriteString(table.table)
	}

	if len(q.conditions) > 0 {
		query.WriteString(" WHERE ")
		for i, condition := range q.conditions {
			if i > 0 {
				if condition.op == opAND {
					query.WriteString(" AND ")
				} else {
					query.WriteString(" OR ")
				}
			}
			query.WriteString(condition.String())
			params = append(params, condition.Params()...)
		}
	}

	if len(q.orderBy) > 0 {
		query.WriteString(" ORDER BY ")
		for i, orderBy := range q.orderBy {
			if i > 0 {
				query.WriteString(", ")
			}
			query.WriteString(orderBy)
		}
	}

	if q.offset != nil {
		query.WriteString(" OFFSET ")
		query.WriteString(strconv.Itoa(*q.offset))
	}

	if q.limit != nil {
		query.WriteString(" LIMIT ")
		query.WriteString(strconv.Itoa(*q.limit))
	}

	queryString := query.String()
	n := 1
	for {
		if strings.Contains(queryString, "?") {
			queryString = strings.Replace(queryString, "?", "$"+strconv.Itoa(n), 1)
			n++
		} else {
			break
		}
	}

	return queryString, params
}
