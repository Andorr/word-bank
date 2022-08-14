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

var WordClasses = []WordClass{
	WordClassNoun,
	WordClassPronoun,
	WordClassVerb,
	WordClassAdjective,
	WordClassAdverb,
	WordClassPreposition,
	WordClassConjunction,
	WordClassInterjection,
	WordClassDeterminer,
	WordClassOther,
}

type Word struct {
	ID           *uuid.UUID     `json:"id"`
	Value        string         `json:"value" validate:"required"`
	Class        WordClass      `json:"class" validate:"required"`
	Tags         []WordTag      `json:"tags"`
	Translations []*Translation `json:"translations" validate:"required"`

	CreatedAt *time.Time `json:"createdAt"`
	UpdatedAt *time.Time `json:"updatedAt"`
}

type Translation struct {
	ID    uuid.UUID `json:"id"`
	Value string    `json:"value" validate:"required"`
}

type WordQueryOptions struct {
	Query *string    `json:"query"`
	Word  *string    `json:"word"`
	Tags  *[]WordTag `json:"tags"`
	Class *WordClass `json:"class"`
}

func (w *WordQueryOptions) Empty() {
	w.Query = new(string)
	w.Word = new(string)
	w.Tags = new([]WordTag)
	w.Class = new(WordClass)
}

type WordUpdateOptions struct {
	ID           uuid.UUID      `json:"id" validate:"required"`
	Value        *string        `json:"value"`
	Class        *WordClass     `json:"class"`
	Tags         []WordTag      `json:"tags"`
	Translations []*Translation `json:"translations"`
}
