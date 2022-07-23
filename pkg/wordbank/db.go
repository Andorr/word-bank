package wordbank

import (
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/Andorr/word-bank/pkg/wordbank/pg"

	"github.com/google/uuid"
)

type DBStore interface {
	// Word
	InsertWord(q *pg.PgContext, word *models.Word) error
	QueryWords(q *pg.PgContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], error)
	UpdateWord(q *pg.PgContext, updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(q *pg.PgContext, id uuid.UUID) error
	GetWordsByIds(q *pg.PgContext, ids []uuid.UUID) ([]*models.Word, error)
	RandomWords(q *pg.PgContext, count int) ([]*models.Word, error)

	// Folders
	InsertFolder(q *pg.PgContext, folder *models.Folder) error
	QueryFolders(q *pg.PgContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], error)
	UpdateFolder(q *pg.PgContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(q *pg.PgContext, id uuid.UUID) error
	GetFolder(q *pg.PgContext, id uuid.UUID) (*models.Folder, error)
}
