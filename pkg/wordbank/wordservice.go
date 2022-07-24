package wordbank

import (
	"fmt"

	"github.com/Andorr/word-bank/pkg/arrayutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"

	"github.com/google/uuid"
)

const (
	ErrFolderNotEmpty ErrorCode = "FOLDER_NOT_EMPTY"
	ErrInvalidWord    ErrorCode = "INVALID_WORD"
	ErrInvalidFolder  ErrorCode = "INVALID_FOLDER"
)

type PgWordService struct {
	DB DBStore
}

func NewPgWordService(dbStore DBStore) *PgWordService {
	return &PgWordService{
		DB: dbStore,
	}
}

func (c *PgWordService) InsertWord(ctx *WordBankContext, word *models.Word) *WordBankError {
	if err := ValidateWord(word); err != nil {
		return errBadRequest(ErrInvalidWord, err)
	}

	return errServerError(c.DB.InsertWord(ctx.pgContext, word))
}

func (c *PgWordService) QueryWords(ctx *WordBankContext, word models.WordQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Word], *WordBankError) {
	return errServerErrorWithValue(c.DB.QueryWords(ctx.pgContext, word, pagination))
}

func (c *PgWordService) GetWord(ctx *WordBankContext, id uuid.UUID) (*models.Word, *WordBankError) {
	words, err := c.DB.GetWordsByIds(ctx.pgContext, []uuid.UUID{id})
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

func (c *PgWordService) UpdateWord(ctx *WordBankContext, updateOptions models.WordUpdateOptions) (*models.Word, *WordBankError) {
	if err := ValidateWordUpdateOptions(updateOptions); err != nil {
		return nil, errBadRequest(ErrInvalidWord, err)
	}

	return errServerErrorWithValue(c.DB.UpdateWord(ctx.pgContext, updateOptions))
}
func (c *PgWordService) DeleteWord(ctx *WordBankContext, id uuid.UUID) *WordBankError {
	err := c.DB.DeleteWord(ctx.pgContext, id)
	if err != nil {
		return errServerError(err)
	}

	folders, err := c.DB.QueryFolders(ctx.pgContext, models.FolderQueryOptions{
		Words: []uuid.UUID{id},
	}, nil)
	if err != nil {
		return errServerError(err)
	}

	for _, folder := range folders.Results {
		if len(folder.Words) == 0 {
			_, err = c.DB.UpdateFolder(ctx.pgContext, models.FolderUpdateOptions{
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
func (c *PgWordService) RandomWords(ctx *WordBankContext, count int) ([]*models.Word, *WordBankError) {
	return errServerErrorWithValue(c.DB.RandomWords(ctx.pgContext, count))
}

// Folders
func (c *PgWordService) InsertFolder(ctx *WordBankContext, folder *models.Folder) *WordBankError {
	if err := ValidateFolder(folder); err != nil {
		return errBadRequest(ErrInvalidFolder, err)
	}

	return errServerError(c.DB.InsertFolder(ctx.pgContext, folder))
}
func (c *PgWordService) QueryFolders(ctx *WordBankContext, folder models.FolderQueryOptions, pagination *models.PaginationOptions) (*models.PageResult[*models.Folder], *WordBankError) {
	return errServerErrorWithValue(c.DB.QueryFolders(ctx.pgContext, folder, pagination))
}
func (c *PgWordService) UpdateFolder(ctx *WordBankContext, updateOptions models.FolderUpdateOptions) (*models.Folder, *WordBankError) {
	if err := ValidateFolderUpdateOptions(updateOptions); err != nil {
		return nil, errBadRequest(ErrInvalidFolder, err)
	}

	return errServerErrorWithValue(c.DB.UpdateFolder(ctx.pgContext, updateOptions))
}
func (c *PgWordService) DeleteFolder(ctx *WordBankContext, id uuid.UUID) *WordBankError {
	// TODO: Check if folder is empty
	folder, err := c.DB.GetFolder(ctx.pgContext, id)
	if err != nil {
		return errServerError(err)
	}

	if len(folder.Words) > 0 {
		return errBadRequest(ErrFolderNotEmpty, fmt.Errorf("folder is not empty"))
	}

	// Check if there are subfolders
	folders, err := c.DB.QueryFolders(ctx.pgContext, models.FolderQueryOptions{
		Parent: &id,
	}, nil)
	if err != nil {
		return errServerError(err)
	}
	if len(folders.Results) > 0 {
		return errBadRequest(ErrFolderNotEmpty, fmt.Errorf("folder is not empty"))
	}

	return errServerError(c.DB.DeleteFolder(ctx.pgContext, id))
}
func (c *PgWordService) GetFolder(ctx *WordBankContext, id uuid.UUID) (*models.Folder, *WordBankError) {
	return errServerErrorWithValue(c.DB.GetFolder(ctx.pgContext, id))
}
func (c *PgWordService) GetFolderContent(ctx *WordBankContext, folderID uuid.UUID) (*models.FolderContent, *WordBankError) {

	folders, err := c.DB.QueryFolders(ctx.pgContext, models.FolderQueryOptions{
		Parent: &folderID,
	}, nil)
	if err != nil {
		return nil, errServerError(err)
	}

	words, err := c.DB.GetWordsByIds(ctx.pgContext,
		arrayutil.Flatten(
			arrayutil.Map(folders.Results, func(folder *models.Folder) []uuid.UUID {
				return folder.Words
			}),
		),
	)
	if err != nil {
		return nil, errServerError(err)
	}

	return &models.FolderContent{
		Folders: folders.Results,
		Words:   words,
	}, nil

}
