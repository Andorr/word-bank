package wordbank

import (
	"wordbank/pkg/wordbank/models"

	"github.com/google/uuid"
)

type WordBank struct {
	Word WordService
}

type WordService interface {
	// Words
	InsertWord(ctx *WordBankContext, word *models.Word) error
	QueryWords(ctx *WordBankContext, word models.WordQueryOptions) (*models.PageResult[models.Word], error)
	UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(ctx *WordBankContext, id uuid.UUID) error
	RandomWords(ctx *WordBankContext, count int) ([]*models.Word, error)

	// Folders
	InsertFolder(ctx *WordBankContext, folder *models.Folder) error
	QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions) (*models.PageResult[models.Folder], error)
	UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(ctx *WordBankContext, id uuid.UUID) error
	GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, error)
	GetFolderContent(ctx *WordBankContext, id uuid.UUID) (models.FolderContent, error)
}

type WordBankContext interface{}

type WordBankError struct {
	Status int
	Err    error
}

func (e WordBankError) Error() string {
	return e.Err.Error()
}
