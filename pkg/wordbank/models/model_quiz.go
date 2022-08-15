package models

import (
	"time"

	"github.com/google/uuid"
)

type QuizQuestionResult struct {
	WordID        uuid.UUID `json:"wordId"`
	NumCorrects   uint64    `json:"numCorrects"`
	NumIncorrects uint64    `json:"numIncorrects"`
}

type QuizResult struct {
	ID        uuid.UUID             `json:"id"`
	Results   []*QuizQuestionResult `json:"results"`
	CreatedAt *time.Time            `json:"createdAt"`
}
