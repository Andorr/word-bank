package pg

import (
	"strconv"
	"strings"

	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
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

func (q Query) WhereIn(column string, values []interface{}) Query {
	q.conditions = append(q.conditions, condition{column + " IN (" + strings.Repeat("?,", len(values)-1) + "?)", opAND, nil, values})
	return q
}

func (q Query) WhereInOR(column string, values []interface{}) Query {
	q.conditions = append(q.conditions, condition{column + " IN (" + strings.Repeat("?,", len(values)-1) + "?)", opOR, nil, values})
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

func (q Query) Pagination(options *models.PaginationOptions, mapper map[string]string, prefix string) Query {
	if options != nil && options.Limit > 0 {
		if options.Page == 0 {
			options.Page = 1
		}
		q = q.Limit(options.Limit).Offset((options.Page - 1) * options.Limit)
	}
	if options != nil && options.OrderBy != "" && mapper[options.OrderByField()] != "" {
		q = q.OrderBy(prefix + mapper[options.OrderByField()] + " " + string(options.OrderByDirection()))
	} else {
		q = q.OrderBy(prefix + "created_at ASC")
	}
	return q
}

func (q Query) GetPage() int {
	if q.offset != nil && q.limit != nil {
		return *q.offset / *q.limit + 1
	}
	return 1
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

	q.buildWhere(&query, &params)

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

	return rebind(query.String()), processParams(params)
}

func (q *Query) buildWhere(query *strings.Builder, params *[]interface{}) {
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
			*params = append(*params, condition.Params()...)
		}
	}
}

func rebind(query string) string {
	n := 1
	for {
		if strings.Contains(query, "?") {
			query = strings.Replace(query, "?", "$"+strconv.Itoa(n), 1)
			n++
		} else {
			break
		}
	}
	return query
}

func processParams(params []interface{}) []interface{} {
	for i, param := range params {
		if value, ok := param.(uuid.UUID); ok {
			params[i] = value.String()
		} else if value, ok := param.(*uuid.UUID); ok {
			params[i] = value.String()
		}
	}
	return params
}

type KeyValue struct {
	Key   string
	Value interface{}
}

type UpdateQuery struct {
	table string

	columns []KeyValue

	returning *string

	query Query
}

func Update(table string) UpdateQuery {
	return UpdateQuery{
		table:   table,
		columns: []KeyValue{},
		query:   NewQuery(table),
	}
}

func (u UpdateQuery) Set(key string, value interface{}) UpdateQuery {
	u.columns = append(u.columns, KeyValue{key, value})
	return u
}

func (u UpdateQuery) Where(where string, parameters ...interface{}) UpdateQuery {
	u.query = u.query.Where(where, parameters...)
	return u
}

func (u UpdateQuery) WhereOr(where string, parameters ...interface{}) UpdateQuery {
	u.query = u.query.WhereOr(where, parameters...)
	return u
}

func (u UpdateQuery) WhereGroup(fn func(c *condition)) UpdateQuery {
	u.query = u.query.WhereGroup(fn)
	return u
}

func (u UpdateQuery) Returning(column string) UpdateQuery {
	u.returning = &column
	return u
}

func (u UpdateQuery) Build() (string, []any) {
	sb := strings.Builder{}
	params := make([]any, 0)

	sb.WriteString("UPDATE ")
	sb.WriteString(u.table)

	if len(u.columns) > 0 {
		sb.WriteString(" SET ")
		for i, column := range u.columns {
			if i > 0 {
				sb.WriteString(", ")
			}
			sb.WriteString(column.Key)
			sb.WriteString(" = ?")
			params = append(params, column.Value)
		}
	}

	u.query.buildWhere(&sb, &params)

	if u.returning != nil {
		sb.WriteString(" RETURNING ")
		sb.WriteString(*u.returning)
	}

	return rebind(sb.String()), processParams(params)
}

type DeleteQuery struct {
	table string

	query Query
}

func Delete(table string) DeleteQuery {
	return DeleteQuery{
		table: table,
		query: NewQuery(table),
	}
}

func (u DeleteQuery) Where(where string, parameters ...interface{}) DeleteQuery {
	u.query = u.query.Where(where, parameters...)
	return u
}

func (u DeleteQuery) WhereOr(where string, parameters ...interface{}) DeleteQuery {
	u.query = u.query.WhereOr(where, parameters...)
	return u
}

func (u DeleteQuery) WhereGroup(fn func(c *condition)) DeleteQuery {
	u.query = u.query.WhereGroup(fn)
	return u
}

func (u DeleteQuery) Build() (string, []any) {
	sb := strings.Builder{}
	params := make([]any, 0)

	sb.WriteString("DELETE FROM ")
	sb.WriteString(u.table)

	u.query.buildWhere(&sb, &params)

	return rebind(sb.String()), processParams(params)
}
