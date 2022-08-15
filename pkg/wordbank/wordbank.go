package wordbank

import (
	"context"
	"fmt"

	"github.com/Andorr/word-bank/internal/pg"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	_ "github.com/lib/pq"

	"github.com/google/uuid"
)

type WordBank struct {
	Word WordService
	Quiz QuizService

	newContextHandler   func(context.Context) (*WordBankContext, error)
	newContextTxHandler func() (*WordBankContext, error)
}

func New(connectionString string) (*WordBank, error) {
	return newWithPG(connectionString)
}

func newWithPG(connectionString string) (*WordBank, error) {

	dbStore, err := pg.NewDBStore(connectionString)
	if err != nil {
		return nil, err
	}

	return &WordBank{
		Word: newWordService(dbStore),
		Quiz: newQuizService(dbStore),
		newContextTxHandler: func() (*WordBankContext, error) {
			tx, err := dbStore.Tx()
			if err != nil {
				return nil, err
			}
			return &WordBankContext{
				value: tx,
				onCommit: func(ctx context.Context) error {
					return dbStore.CommitContext(ctx)
				},
				onRollback: func(ctx context.Context) error {
					return dbStore.Rollback(ctx)
				},
			}, err
		},
		newContextHandler: func(ctx context.Context) (*WordBankContext, error) {
			conn, err := dbStore.Conn(ctx)
			if err != nil {
				return nil, err
			}
			return &WordBankContext{
				value: conn,
			}, nil
		},
	}, nil
}

func (wb *WordBank) NewContext(ctx context.Context) (*WordBankContext, error) {
	if wb.newContextHandler == nil {
		return nil, errServerError(fmt.Errorf("no context available"))
	}

	return wb.newContextHandler(ctx)
}

func (wb *WordBank) RunTx(fn func(ctx *WordBankContext) error) error {
	ctx, err := wb.newContextTxHandler()
	if err != nil {
		return err
	}

	err = fn(ctx)
	if err != nil {
		ctx.rollback()
	} else {
		ctx.commit()
	}
	return err
}

type WordService interface {
	// Words
	InsertWord(ctx *WordBankContext, word *models.Word) error
	QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], error)
	GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, error)
	GetWordsByIDs(ctx *WordBankContext, ids []uuid.UUID) ([]*models.Word, error)
	UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(ctx *WordBankContext, id uuid.UUID) error
	RandomWords(ctx *WordBankContext, count int) ([]*models.Word, error)

	// Folders
	InsertFolder(ctx *WordBankContext, folder *models.Folder) error
	QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], error)
	UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(ctx *WordBankContext, id uuid.UUID) error
	GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, error)
	GetFolderContent(ctx *WordBankContext, folderID uuid.UUID) (*models.FolderContent, error)
}

type QuizService interface {
	InsertQuizResult(ctx *WordBankContext, result *models.QuizResult) error
}
