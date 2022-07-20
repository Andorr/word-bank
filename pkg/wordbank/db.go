package wordbank

import (
	"wordbank/pkg/wordbank/models"

	"github.com/google/uuid"
)

type DBStore interface {
	// Word
	InsertWord(word *models.Word) error
	QueryWords(word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Word], error)
	UpdateWord(updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(id uuid.UUID) error
	GetWordsByIds(ids []uuid.UUID) ([]*models.Word, error)
	RandomWords(count int) ([]*models.Word, error)

	// Folders
	InsertFolder(folder *models.Folder) error
	QueryFolders(folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Folder], error)
	UpdateFolder(updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(id uuid.UUID) error
	GetFolder(id uuid.UUID) (*models.Folder, error)
}
