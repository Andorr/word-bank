package echoutil

import (
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/labstack/echo/v4"
)

func BindPaginationOptions(c echo.Context) (*models.PaginationOptions, error) {

	paginationOptions := &models.PaginationOptions{}

	err := echo.QueryParamsBinder(c).
		Int("page", &paginationOptions.Page).
		Int("limit", &paginationOptions.Limit).
		BindError()

	return paginationOptions, err
}

func BindWordQueryOptions(c echo.Context) (*models.WordQueryOptions, error) {

	options := struct {
		Query *string           `query:"query"`
		Word  *string           `query:"word"`
		Class *models.WordClass `query:"class"`
		Tags  []string
	}{}

	err := (&echo.DefaultBinder{}).BindQueryParams(c, &options)
	if err != nil {
		return nil, err
	}

	// String slices are an unknown type for the echo.DefaultBinder.
	err = echo.QueryParamsBinder(c).
		Strings("tags", &options.Tags).
		BindError()
	if err != nil {
		return nil, err
	}

	opt := &models.WordQueryOptions{
		Query: options.Query,
		Word:  options.Word,
		Class: options.Class,
	}
	if len(options.Tags) > 0 {
		opt.Tags = &options.Tags
	}
	return opt, nil
}
