package wordbank

import "github.com/Andorr/word-bank/pkg/wordbank/models"

type quizServiceImpl struct {
	dbStore DBStore
}

func newQuizService(dbStore DBStore) *quizServiceImpl {
	return &quizServiceImpl{dbStore: dbStore}
}

func (s *quizServiceImpl) InsertQuizResult(ctx *WordBankContext, quizResult *models.QuizResult) error {
	return errServerError(s.dbStore.InsertQuizResult(ctx, quizResult))
}
