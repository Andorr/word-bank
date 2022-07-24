package wordbank

import (
	"context"

	"github.com/Andorr/word-bank/internal/pg"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	_ "github.com/lib/pq"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"
)

type WordBank struct {
	Word WordService

	newContextHandler func(context.Context) (*WordBankContext, error)
}

func NewWithPG(connectionString string) (*WordBank, error) {
	db, err := sqlx.Connect("postgres", connectionString)
	if err != nil {
		return nil, err
	}

	dbStore := pg.NewDBStore(db)

	return &WordBank{
		Word: NewPgWordService(dbStore),
		newContextHandler: func(ctx context.Context) (*WordBankContext, error) {
			pgContext, err := dbStore.NewContext(ctx)
			return &WordBankContext{
				pgContext: pgContext,
			}, err
		},
	}, nil
}

func (wb *WordBank) NewContext(ctx context.Context) (*WordBankContext, error) {
	return wb.newContextHandler(ctx)
}

type WordService interface {
	// Words
	InsertWord(ctx *WordBankContext, word *models.Word) *WordBankError
	QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], *WordBankError)
	GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, *WordBankError)
	UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, *WordBankError)
	DeleteWord(ctx *WordBankContext, id uuid.UUID) *WordBankError
	RandomWords(ctx *WordBankContext, count int) ([]*models.Word, *WordBankError)

	// Folders
	InsertFolder(ctx *WordBankContext, folder *models.Folder) *WordBankError
	QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], *WordBankError)
	UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, *WordBankError)
	DeleteFolder(ctx *WordBankContext, id uuid.UUID) *WordBankError
	GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, *WordBankError)
	GetFolderContent(ctx *WordBankContext, folderID uuid.UUID) (*models.FolderContent, *WordBankError)
}

type WordBankContext struct {
	pgContext *pg.PgContext
}

func (ctx *WordBankContext) Commit() error {
	return ctx.pgContext.Commit()
}
