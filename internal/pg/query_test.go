package pg

import (
	"fmt"
	"testing"

	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
	"github.com/lib/pq"
	"github.com/stretchr/testify/assert"
)

func TestQueryBuilder(t *testing.T) {

	valueUUID := uuid.New()

	qb := NewQuery("words").
		CrossJoin("UNNEST(translations) as t").
		WhereGroup(func(c *condition) {
			c.Where("value SIMILAR TO '%?%'", "query").
				WhereOr("t.value SIMILAR TO '%?%'", "query")
		}).
		WhereOr("class = ?", models.WordClassNoun).
		WhereOr("tags @> ?", pq.Array([]string{"hello"})).
		WhereOr("value = ?", "word").
		WhereOr("value = ?", valueUUID).
		WhereOr("value = ?", &valueUUID)

	qbCount := qb.Count("*")

	qb = qb.Limit(10).Offset(10)

	query, params := qb.Build()

	expectedWhere := "WHERE (value SIMILAR TO '%$1%' OR t.value SIMILAR TO '%$2%') OR class = $3 OR tags @> $4 OR value = $5 OR value = $6 OR value = $7"

	assert.Equal(t, fmt.Sprintf("SELECT * FROM words, UNNEST(translations) as t %s OFFSET 10 LIMIT 10", expectedWhere), query)
	if assert.Equal(t, 7, len(params)) {
		assert.Equal(t, "query", params[0])
		assert.Equal(t, "query", params[1])
		assert.Equal(t, models.WordClassNoun, params[2])
		assert.Equal(t, pq.Array([]string{"hello"}), params[3])
		assert.Equal(t, "word", params[4])
		assert.Equal(t, valueUUID.String(), params[5])
		assert.Equal(t, valueUUID.String(), params[6])
	}

	query, params = qbCount.Build()
	assert.Equal(t, fmt.Sprintf("SELECT COUNT(*) FROM words, UNNEST(translations) as t %s", expectedWhere), query)
	if assert.Equal(t, 7, len(params)) {
		assert.Equal(t, "query", params[0])
		assert.Equal(t, "query", params[1])
		assert.Equal(t, models.WordClassNoun, params[2])
		assert.Equal(t, pq.Array([]string{"hello"}), params[3])
		assert.Equal(t, "word", params[4])
		assert.Equal(t, valueUUID.String(), params[5])
		assert.Equal(t, valueUUID.String(), params[6])
	}
}

func TestUpdateQuery(t *testing.T) {

	query, params := Update("words").
		Set("value", "new-value").
		Set("class", models.WordClassNoun).
		Where("value = ?", "old-value").
		Build()

	assert.Equal(t, "UPDATE words SET value = $1, class = $2 WHERE value = $3", query)
	if assert.Equal(t, 3, len(params)) {
		assert.Equal(t, "new-value", params[0])
		assert.Equal(t, models.WordClassNoun, params[1])
		assert.Equal(t, "old-value", params[2])
	}
}
