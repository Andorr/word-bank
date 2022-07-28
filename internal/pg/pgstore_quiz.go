package pg

import (
	"context"

	pgmodels "github.com/Andorr/word-bank/internal/pg/models"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
)

func (s *PgDBStore) InsertQuizResult(ctx context.Context, quizResult *models.QuizResult) error {

	pgQuizResult := pgmodels.PgQuizQuestionResultFrom(quizResult)

	err := s.driver(ctx).
		QueryRowx("INSERT INTO words (results) VALUES ($1) RETURNING *", pgQuizResult).
		StructScan(pgQuizResult)
	if err != nil {
		return err
	}

	pgQuizResult.IntoQuizResult(quizResult)
	return nil
}
