package models

import (
	"time"

	"github.com/google/uuid"
)

type WordTag = string
type WordClass string

const (
	WordClassNone         WordClass = "NONE"
	WordClassNoun         WordClass = "NOUN"
	WordClassPronoun      WordClass = "PRONOUN"
	WordClassVerb         WordClass = "VERB"
	WordClassAdjective    WordClass = "ADJECTIVE"
	WordClassAdverb       WordClass = "ADVERB"
	WordClassPreposition  WordClass = "PREPOSITION"
	WordClassConjunction  WordClass = "CONJUNCTION"
	WordClassInterjection WordClass = "INTERJECTION"
	WordClassDeterminer   WordClass = "DETERMINER"
	WordClassOther        WordClass = "OTHER"
)

type Word struct {
	ID           *uuid.UUID     `json:"id"`
	Value        string         `json:"value"`
	Class        WordClass      `json:"class"`
	Tags         []WordTag      `json:"tags"`
	Translations []*Translation `json:"translations"`

	CreatedAt *time.Time `json:"createdAt"`
	UpdatedAt *time.Time `json:"updatedAt"`
}

type Translation struct {
	ID    uuid.UUID
	Value string
}

type WordQueryOptions struct {
	Query *string
	Word  *string
	Tags  *[]WordTag
	Class *WordClass
}

type WordUpdateOptions struct {
	ID           uuid.UUID
	Word         *string
	Class        *WordClass
	Tags         []WordTag
	Translations []*Translation
}
