package pgmodels

import (
	"database/sql/driver"
	"fmt"
	"strings"
	"time"

	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/internal/pg/pgutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
	"github.com/lib/pq"
)

type PgTranslation struct {
	ID  uuid.UUID `db:"id"`
	Val string    `db:"value"`
}

func (t *PgTranslation) Value() (driver.Value, error) {
	return pgutil.SliceToPgValue([]interface{}{t.ID, t.Val})
}

var _ pgutil.StringScan[*PgTranslation] = (*PgTranslation)(nil)

func (t *PgTranslation) ScanString(value string) (*PgTranslation, error) {
	raw := strings.Split(value, ",")
	if len(raw) != 2 {
		return nil, fmt.Errorf("unable to parse %s", value)
	}

	id, err := uuid.Parse(raw[0])
	if err != nil {
		return nil, err
	}

	return &PgTranslation{
		ID:  id,
		Val: strings.Trim(raw[1], "'"),
	}, nil
}

type PgTranslationSlice []*PgTranslation

func (pts *PgTranslationSlice) Scan(value interface{}) error {
	if value == nil {
		*pts = nil
		return nil
	}

	var arr []*PgTranslation
	err := pgutil.PgScanArray(&arr, value)
	*pts = arr
	return err
}

type PgWord struct {
	ID           *uuid.UUID         `db:"id"`
	Value        string             `db:"value"`
	Class        models.WordClass   `db:"class"`
	Tags         pq.StringArray     `db:"tags"`
	Translations PgTranslationSlice `db:"translations"`

	CreatedAt *time.Time `db:"created_at"`
	UpdatedAt *time.Time `db:"updated_at"`
}

var PgWordColumns map[string]string = map[string]string{
	"id":           "id",
	"value":        "value",
	"class":        "class",
	"tags":         "tags",
	"translations": "translations",
	"createdAt":    "created_at",
	"updatedAt":    "updated_at",
}

func PgWordFrom(other *models.Word) PgWord {
	return PgWord{
		ID:    other.ID,
		Value: other.Value,
		Class: other.Class,
		Tags:  arrayutil.ValueOrDefault(other.Tags, []string{}),
		Translations: arrayutil.Map(
			arrayutil.ValueOrDefault(other.Translations, []*models.Translation{}),
			func(o *models.Translation) *PgTranslation {
				return &PgTranslation{
					ID:  o.ID,
					Val: o.Value,
				}
			},
		),
		CreatedAt: other.CreatedAt,
		UpdatedAt: other.UpdatedAt,
	}
}

func (w *PgWord) IntoWord(word *models.Word) *models.Word {
	word.ID = w.ID
	word.Value = w.Value
	word.Class = w.Class
	word.Tags = w.Tags
	word.Translations = arrayutil.Map(w.Translations, func(o *PgTranslation) *models.Translation {
		return &models.Translation{
			ID:    o.ID,
			Value: o.Val,
		}
	})
	word.CreatedAt = w.CreatedAt
	word.UpdatedAt = w.UpdatedAt
	return word
}

func (w *PgWord) ToWord() *models.Word {
	return w.IntoWord(&models.Word{})
}
