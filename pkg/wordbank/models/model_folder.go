package models

import (
	"time"

	"github.com/google/uuid"
)

type Folder struct {
	ID     *uuid.UUID  `json:"id"`
	Name   string      `json:"name" validate:"required"`
	Parent *uuid.UUID  `json:"parent"`
	Words  []uuid.UUID `json:"words"`

	CreatedAt *time.Time `json:"createdAt"`
	UpdatedAt *time.Time `json:"updatedAt"`
}

type FolderContent struct {
	Words   []*Word   `json:"words"`
	Folders []*Folder `json:"folders"`
}

type FolderQueryOptions struct {
	Query  *string     `json:"query"`
	Words  []uuid.UUID `json:"words"`
	Parent *uuid.UUID  `json:"parent"`
	IDs    []uuid.UUID `json:"ids"`
}

type FolderUpdateOptions struct {
	ID     uuid.UUID   `json:"id" validate:"required"`
	Name   *string     `json:"name"`
	Parent *uuid.UUID  `json:"parent"`
	Add    []uuid.UUID `json:"add"`
	Remove []uuid.UUID `json:"remove"`
}
