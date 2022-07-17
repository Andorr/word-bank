package wordbank

import (
	"wordbank/pkg/wordbank/models"
	"wordbank/pkg/wordbank/pg"

	_ "github.com/lib/pq"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
)

type WordBank struct {
	Word WordService
}

func NewWithPG(connectionString string) (*WordBank, error) {
	db, err := sqlx.Connect("postgres", connectionString)
	if err != nil {
		return nil, err
	}

	dbStore := pg.NewDBStore(db)

	return &WordBank{
		Word: NewWordService(dbStore),
	}, nil
}

type WordService interface {
	// Words
	InsertWord(ctx *WordBankContext, word *models.Word) error
	QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Word], error)
	GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, error)
	UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(ctx *WordBankContext, id uuid.UUID) error
	RandomWords(ctx *WordBankContext, count int) ([]*models.Word, error)

	// Folders
	InsertFolder(ctx *WordBankContext, folder *models.Folder) error
	QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Folder], error)
	UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(ctx *WordBankContext, id uuid.UUID) error
	GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, error)
	GetFolderContent(ctx *WordBankContext, id uuid.UUID) (*models.FolderContent, error)
}

type WordBankContext interface{}

type WordBankError struct {
	Status int
	Err    error
}

func (e WordBankError) Error() string {
	return e.Err.Error()
}
