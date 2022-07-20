package pgmodels

import (
	"database/sql/driver"
	"fmt"
	"strings"
	"time"
	"wordbank/pkg/util/arrayutil"
	"wordbank/pkg/wordbank/models"
	"wordbank/pkg/wordbank/pg/utils"

	"github.com/google/uuid"
	"github.com/lib/pq"
)

type PgTranslation struct {
	ID  uuid.UUID `db:"id"`
	Val string    `db:"value"`
}

func (t *PgTranslation) Value() (driver.Value, error) {
	return utils.SliceToPgValue([]interface{}{t.ID, t.Val})
}

type PgTranslationSlice []*PgTranslation

func (pts *PgTranslationSlice) Scan(value interface{}) error {
	if value == nil {
		*pts = nil
		return nil
	}

	source, ok := value.([]byte)
	if !ok {
		return fmt.Errorf("unable to convert %T to PgTranslationSlice", value)
	}

	var pgTranslations PgTranslationSlice
	tokens := strings.Split(string(source), "\",\"")
	for _, token := range tokens {
		// Remove unecessary tokens
		for _, unwanted := range []string{"\\\"", "\"", "{", "}", "(", ")"} {
			token = strings.ReplaceAll(token, unwanted, "")
		}

		raw := strings.Split(token, ",")

		if len(raw) != 2 {
			return fmt.Errorf("unable to parse %s", token)
		}

		id, err := uuid.Parse(raw[0])
		if err != nil {
			return err
		}

		pgTranslations = append(pgTranslations, &PgTranslation{
			ID:  id,
			Val: strings.Trim(raw[1], "'"),
		})
	}

	*pts = pgTranslations
	return nil
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

// func (w *PgWord) Value() (driver.Value, error) {
// 	return utils.SliceToPgValue([]interface{}{w.ID, w.Val, w.Type, w.Tags, w.Translations, w.CreatedAt, w.UpdatedAt})
// }

func PgWordFrom(other *models.Word) PgWord {
	return PgWord{
		ID:    other.ID,
		Value: other.Value,
		Class: other.Class,
		Tags:  other.Tags,
		Translations: arrayutil.Map(other.Translations, func(o *models.Translation) *PgTranslation {
			return &PgTranslation{
				ID:  o.ID,
				Val: o.Value,
			}
		}),
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
