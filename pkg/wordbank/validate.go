package wordbank

import (
	"fmt"

	"github.com/Andorr/word-bank/pkg/arrayutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/go-playground/validator"
	"github.com/google/uuid"
)

var validate *validator.Validate

func init() {
	validate = validator.New()
}

func ValidateWord(word *models.Word) error {
	err := validate.Struct(word)
	if err != nil {
		return err
	}

	if !arrayutil.Contains(models.WordClasses, word.Class) {
		return fmt.Errorf("invalid word class: %s", word.Class)
	}

	return nil
}

func ValidateWordUpdateOptions(updateOptions models.WordUpdateOptions) error {
	err := validate.Struct(updateOptions)
	if err != nil {
		return err
	}

	if updateOptions.ID == uuid.Nil {
		return fmt.Errorf("invalid word id")
	}

	if updateOptions.Class != nil && !arrayutil.Contains(models.WordClasses, *updateOptions.Class) {
		return fmt.Errorf("invalid word class: %s", *updateOptions.Class)
	}

	return nil
}

func ValidateFolder(folder *models.Folder) error {
	return validate.Struct(folder)
}

func ValidateFolderUpdateOptions(updateOptions models.FolderUpdateOptions) error {
	return validate.Struct(updateOptions)
}
