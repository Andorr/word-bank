package wordbank

import (
	"fmt"
	"wordbank/pkg/util/arrayutil"
	"wordbank/pkg/wordbank/models"

	"github.com/google/uuid"
)

type WordServiceImpl struct {
	DB DBStore
}

func NewWordService(dbStore DBStore) *WordServiceImpl {
	return &WordServiceImpl{
		DB: dbStore,
	}
}

func (c *WordServiceImpl) InsertWord(ctx *WordBankContext, word *models.Word) error {
	return c.DB.InsertWord(word)
}

func (c *WordServiceImpl) QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Word], error) {
	return c.DB.QueryWords(word, pagination)
}

func (c *WordServiceImpl) GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, error) {
	words, err := c.DB.GetWordsByIds([]uuid.UUID{id})
	if err != nil {
		return nil, err
	}

	word := arrayutil.Find(words, func(word *models.Word) bool {
		return *word.ID == id
	})
	if word == nil {
		return nil, WordBankError{Status: 404, Err: fmt.Errorf("word not found")}
	}
	return *word, nil
}

func (c *WordServiceImpl) UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, error) {
	return nil, nil
}
func (c *WordServiceImpl) DeleteWord(ctx *WordBankContext, id uuid.UUID) error { return nil }
func (c *WordServiceImpl) RandomWords(ctx *WordBankContext, count int) ([]*models.Word, error) {
	return nil, nil
}

// Folders
func (c *WordServiceImpl) InsertFolder(ctx *WordBankContext, folder *models.Folder) error { return nil }
func (c *WordServiceImpl) QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[models.Folder], error) {
	return nil, nil
}
func (c *WordServiceImpl) UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error) {
	return nil, nil
}
func (c *WordServiceImpl) DeleteFolder(ctx *WordBankContext, id uuid.UUID) error {
	// TODO: Check if folder is empty
	return nil
}
func (c *WordServiceImpl) GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, error) {
	return nil, nil
}
func (c *WordServiceImpl) GetFolderContent(ctx *WordBankContext, id uuid.UUID) (*models.FolderContent, error) {
	return nil, nil
}
