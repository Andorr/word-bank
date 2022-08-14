package pgmodels

import (
	"time"

	"github.com/Andorr/word-bank/internal/pg/pgutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
	"github.com/lib/pq"
)

type PgFolder struct {
	ID     *uuid.UUID     `db:"id"`
	Name   string         `db:"name"`
	Parent *uuid.UUID     `db:"parent"`
	Words  pq.StringArray `db:"words"` // This is an []uuid.UUID. Using StringArray as scanner is not implemented for uuid arrays.

	CreatedAt *time.Time `db:"created_at"`
	UpdatedAt *time.Time `db:"updated_at"`
}

var PgFolderColumns = map[string]string{
	"id":        "id",
	"name":      "name",
	"parent":    "parent",
	"words":     "words",
	"createdAt": "created_at",
	"updatedAt": "updated_at",
}

func PgFolderFrom(other *models.Folder) PgFolder {
	return PgFolder{
		ID:        other.ID,
		Name:      other.Name,
		Parent:    other.Parent,
		Words:     pgutil.UUIDArrayToString(other.Words),
		CreatedAt: other.CreatedAt,
		UpdatedAt: other.UpdatedAt,
	}
}

func (f *PgFolder) IntoFolder(folder *models.Folder) *models.Folder {
	folder.ID = f.ID
	folder.Name = f.Name
	folder.Parent = f.Parent
	folder.Words = pgutil.StringArrayToUUID(f.Words)
	folder.CreatedAt = f.CreatedAt
	folder.UpdatedAt = f.UpdatedAt
	return folder
}

func (f *PgFolder) ToFolder() *models.Folder {
	return f.IntoFolder(&models.Folder{})
}
