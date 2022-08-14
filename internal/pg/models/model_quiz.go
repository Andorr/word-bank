package pgmodels

import (
	"database/sql"
	"database/sql/driver"
	"fmt"
	"strconv"
	"strings"
	"time"

	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/internal/pg/pgutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
)

type PgQuizQuestionResult struct {
	WordID        uuid.UUID `db:"word_id"`
	NumCorrects   uint64    `db:"num_corrects"`
	NumIncorrects uint64    `db:"num_incorrects"`
}

type PgQuizResult struct {
	ID        uuid.UUID                 `db:"id"`
	Results   PgQuizQuestionResultSlice `db:"results"`
	CreatedAt *time.Time                `db:"created_at"`
}

func (t *PgQuizQuestionResult) Value() (driver.Value, error) {
	return pgutil.SliceToPgValue([]interface{}{t.WordID, t.NumCorrects, t.NumIncorrects})
}

func (t *PgQuizQuestionResult) Scan(value interface{}) error {
	fmt.Println(string(value.([]uint8)))
	return nil
}

func (t *PgQuizQuestionResult) ScanString(value string) (*PgQuizQuestionResult, error) {
	tokens := strings.Split(value, ",")
	if len(tokens) != 3 {
		return nil, fmt.Errorf("unable to parse %s", value)
	}

	id, err := uuid.Parse(tokens[0])
	if err != nil {
		return nil, err
	}

	numCorrects, err := strconv.Atoi(tokens[1])
	if err != nil {
		return nil, err
	}

	numIncorrects, err := strconv.Atoi(tokens[2])
	if err != nil {
		return nil, err
	}

	return &PgQuizQuestionResult{
		WordID:        id,
		NumCorrects:   uint64(numCorrects),
		NumIncorrects: uint64(numIncorrects),
	}, nil
}

type PgQuizQuestionResultSlice []*PgQuizQuestionResult

var _ sql.Scanner = (*PgQuizQuestionResultSlice)(nil)

func (pts *PgQuizQuestionResultSlice) Scan(value interface{}) error {

	if value == nil {
		*pts = nil
		return nil
	}

	var quizResult []*PgQuizQuestionResult
	err := pgutil.PgScanArray(&quizResult, value)
	if err != nil {
		return err
	}
	*pts = quizResult
	return nil
}

func PgQuizQuestionResultFrom(other *models.QuizResult) PgQuizResult {
	return PgQuizResult{
		ID: other.ID,
		Results: arrayutil.Map(other.Results, func(q *models.QuizQuestionResult) *PgQuizQuestionResult {
			return &PgQuizQuestionResult{
				WordID:        q.WordID,
				NumCorrects:   q.NumCorrects,
				NumIncorrects: q.NumIncorrects,
			}
		}),
		CreatedAt: other.CreatedAt,
	}
}

func (q *PgQuizResult) IntoQuizResult(other *models.QuizResult) *models.QuizResult {
	other.ID = q.ID
	other.Results = arrayutil.Map(q.Results, func(q *PgQuizQuestionResult) *models.QuizQuestionResult {
		return &models.QuizQuestionResult{
			WordID:        q.WordID,
			NumCorrects:   q.NumCorrects,
			NumIncorrects: q.NumIncorrects,
		}
	})
	other.CreatedAt = q.CreatedAt
	return other
}
