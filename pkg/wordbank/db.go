package wordbank

import (
	"context"

	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
)

type DBStore interface {
	// Word
	InsertWord(q context.Context, word *models.Word) error
	QueryWords(q context.Context, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], error)
	UpdateWord(q context.Context, updateOptions models.WordUpdateOptions) (*models.Word, error)
	DeleteWord(q context.Context, id uuid.UUID) error
	GetWordsByIds(q context.Context, ids []uuid.UUID) ([]*models.Word, error)
	RandomWords(q context.Context, count int) ([]*models.Word, error)

	// Folders
	InsertFolder(q context.Context, folder *models.Folder) error
	QueryFolders(q context.Context, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], error)
	UpdateFolder(q context.Context, updateOptions models.FolderUpdateOptions) (*models.Folder, error)
	DeleteFolder(q context.Context, id uuid.UUID) error
	GetFolder(q context.Context, id uuid.UUID) (*models.Folder, error)

	// Quiz
	InsertQuizResult(q context.Context, quiz *models.QuizResult) error
}
