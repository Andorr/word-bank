package models

import (
	"time"

	"github.com/google/uuid"
)

type WordTag = string
type WordType string

type Word struct {
	ID           uuid.UUID      `json:"id"`
	Value        string         `json:"value"`
	Type         WordType       `json:"type"`
	Tags         []WordTag      `json:"tags"`
	Translations []*Translation `json:"translations"`

	CreatedAt *time.Time `json:"createdAt"`
	UpdatedAt *time.Time `json:"updatedAt"`
}

type Translation struct {
	ID    *uuid.UUID
	Value string
}

type WordQueryOptions struct {
	Query *string
	Word  *string
	Tags  *[]WordTag
	Type  *WordType
}

type WordUpdateOptions struct {
	ID          uuid.UUID
	Word        *string
	Kind        *WordType
	Tags        []WordTag
	Translation []*Translation
}
