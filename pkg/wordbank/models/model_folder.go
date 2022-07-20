package models

import (
	"time"

	"github.com/google/uuid"
)

type Folder struct {
	ID     *uuid.UUID  `json:"id"`
	Name   string      `json:"name"`
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
	Query  *string
	Words  []uuid.UUID
	Parent *uuid.UUID
	IDs    []uuid.UUID
}

type FolderUpdateOptions struct {
	ID     uuid.UUID
	Name   *string
	Parent *uuid.UUID
	Add    []uuid.UUID
	Remove []uuid.UUID
}
