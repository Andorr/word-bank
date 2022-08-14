package wordbank

import (
	"fmt"

	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
)

const (
	ErrFolderNotEmpty  ErrorCode = "FOLDER_NOT_EMPTY"
	ErrInvalidWord     ErrorCode = "INVALID_WORD"
	ErrInvalidFolder   ErrorCode = "INVALID_FOLDER"
	ErrInvalidCount    ErrorCode = "INVALID_COUNT"
	ErrParentNotExists ErrorCode = "PARENT_NOT_EXISTS"
	ErrWordsNotExists  ErrorCode = "WORDS_NOT_EXISTS"
)

type wordServiceImpl struct {
	DB DBStore
}

var _ WordService = (*wordServiceImpl)(nil)

func newWordService(dbStore DBStore) *wordServiceImpl {
	return &wordServiceImpl{
		DB: dbStore,
	}
}

func (c *wordServiceImpl) InsertWord(ctx *WordBankContext, word *models.Word) error {
	if err := ValidateWord(word); err != nil {
		return errBadRequest(ErrInvalidWord, err)
	}

	return errServerError(c.DB.InsertWord(ctx, word))
}

func (c *wordServiceImpl) QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], error) {
	return errServerErrorWithValue(c.DB.QueryWords(ctx, word, pagination))
}

func (c *wordServiceImpl) GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, error) {
	words, err := c.DB.GetWordsByIds(ctx, []uuid.UUID{id})
	if err != nil {
		return nil, errServerError(err)
	}

	word := arrayutil.Find(words, func(word *models.Word) bool {
		return *word.ID == id
	})
	if word == nil {
		return nil, errNotFound(fmt.Errorf("word not found"))
	}
	return *word, nil
}

func (c *wordServiceImpl) GetWordsByIDs(ctx *WordBankContext, ids []uuid.UUID) ([]*models.Word, error) {
	return errServerErrorWithValue(c.DB.GetWordsByIds(ctx, ids))
}

func (c *wordServiceImpl) UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, error) {
	if err := ValidateWordUpdateOptions(updateOptions); err != nil {
		return nil, errBadRequest(ErrInvalidWord, err)
	}

	return errServerErrorWithValue(c.DB.UpdateWord(ctx, updateOptions))
}
func (c *wordServiceImpl) DeleteWord(ctx *WordBankContext, id uuid.UUID) error {
	err := c.DB.DeleteWord(ctx, id)
	if err != nil {
		return errServerError(err)
	}

	folders, err := c.DB.QueryFolders(ctx, models.FolderQueryOptions{
		Words: []uuid.UUID{id},
	}, nil)
	if err != nil {
		return errServerError(err)
	}

	for _, folder := range folders.Results {
		if len(folder.Words) > 0 {
			_, err = c.DB.UpdateFolder(ctx, models.FolderUpdateOptions{
				ID:     *folder.ID,
				Remove: []uuid.UUID{id},
			})
			if err != nil {
				return errServerError(err)
			}
		}
	}

	return nil
}
func (c *wordServiceImpl) RandomWords(ctx *WordBankContext, count int) ([]*models.Word, error) {
	if count < 1 {
		return nil, errBadRequest(ErrInvalidCount, fmt.Errorf("count must be greater than 0"))
	}

	return errServerErrorWithValue(c.DB.RandomWords(ctx, count))
}

// Folders
func (c *wordServiceImpl) InsertFolder(ctx *WordBankContext, folder *models.Folder) error {
	if err := ValidateFolder(folder); err != nil {
		return errBadRequest(ErrInvalidFolder, err)
	}

	// Check if parent exists
	if folder.Parent != nil {
		parent, err := c.DB.GetFolder(ctx, *folder.Parent)
		if err != nil {
			return errServerError(err)
		} else if parent == nil {
			return errBadRequest(ErrParentNotExists, fmt.Errorf("parent folder not found"))
		}
	}

	if len(folder.Words) > 0 {
		words, err := c.DB.GetWordsByIds(ctx, folder.Words)
		if err != nil {
			return errServerError(err)
		}
		if len(words) != len(folder.Words) {
			return errBadRequest(ErrWordsNotExists, fmt.Errorf("words not found"))
		}
	}

	return errServerError(c.DB.InsertFolder(ctx, folder))
}
func (c *wordServiceImpl) QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], error) {
	return errServerErrorWithValue(c.DB.QueryFolders(ctx, folder, pagination))
}
func (c *wordServiceImpl) UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, error) {
	if err := ValidateFolderUpdateOptions(updateOptions); err != nil {
		return nil, errBadRequest(ErrInvalidFolder, err)
	}

	return errServerErrorWithValue(c.DB.UpdateFolder(ctx, updateOptions))
}
func (c *wordServiceImpl) DeleteFolder(ctx *WordBankContext, id uuid.UUID) error {
	// TODO: Check if folder is empty
	folder, err := c.DB.GetFolder(ctx, id)
	if err != nil {
		return errServerError(err)
	}

	if len(folder.Words) > 0 {
		return errBadRequest(ErrFolderNotEmpty, fmt.Errorf("folder is not empty"))
	}

	// Check if there are subfolders
	folders, err := c.DB.QueryFolders(ctx, models.FolderQueryOptions{
		Parent: &id,
	}, nil)
	if err != nil {
		return errServerError(err)
	}
	if len(folders.Results) > 0 {
		return errBadRequest(ErrFolderNotEmpty, fmt.Errorf("folder is not empty"))
	}

	return errServerError(c.DB.DeleteFolder(ctx, id))
}
func (c *wordServiceImpl) GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, error) {
	return errServerErrorWithValue(c.DB.GetFolder(ctx, id))
}
func (c *wordServiceImpl) GetFolderContent(ctx *WordBankContext, folderID uuid.UUID) (*models.FolderContent, error) {

	folder, err := c.DB.GetFolder(ctx, folderID)
	if err != nil {
		return nil, errServerError(err)
	}

	folders, err := c.DB.QueryFolders(ctx, models.FolderQueryOptions{
		Parent: &folderID,
	}, nil)
	if err != nil {
		return nil, errServerError(err)
	}

	words, err := c.DB.GetWordsByIds(ctx, folder.Words)
	if err != nil {
		return nil, errServerError(err)
	}

	return &models.FolderContent{
		Folders: folders.Results,
		Words:   words,
	}, nil

}
