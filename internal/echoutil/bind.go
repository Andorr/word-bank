package echoutil

import (
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/labstack/echo/v4"
)

func BindWord(e echo.Context) (*models.Word, error) {
	word := new(models.Word)
	if err := e.Bind(word); err != nil {
		e.Error(err)
		return nil, err
	}
	return word, nil
}

func BindWordUpdateOptions(e echo.Context) (*models.WordUpdateOptions, error) {
	options := new(models.WordUpdateOptions)
	if err := e.Bind(options); err != nil {
		e.Error(err)
		return nil, err
	}
	return options, nil
}
