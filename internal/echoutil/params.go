package echoutil

import (
	"fmt"

	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
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

func BindFolderQueryOptions(c echo.Context) (*models.FolderQueryOptions, error) {
	options := struct {
		Query  *string    `query:"query"`
		Parent *uuid.UUID `query:"parent"`
		Words  []string
		IDs    []string
	}{}

	err := (&echo.DefaultBinder{}).BindQueryParams(c, &options)
	if err != nil {
		return nil, err
	}

	err = echo.QueryParamsBinder(c).
		Strings("words", &options.Words).
		Strings("ids", &options.IDs).
		BindError()

	opt := &models.FolderQueryOptions{
		Query:  options.Query,
		Parent: options.Parent,
	}

	if len(options.Words) > 0 {
		stringsToUUIDs := func(strings []string) []uuid.UUID {
			return arrayutil.Filter(arrayutil.Map(options.IDs, func(word string) uuid.UUID {
				id, err := uuid.Parse(word)
				if err != nil {
					return uuid.Nil
				}
				return id
			}), func(id uuid.UUID) bool {
				return id != uuid.Nil
			})
		}

		opt.Words = stringsToUUIDs(options.Words)
		opt.IDs = stringsToUUIDs(options.IDs)
	}

	return opt, nil
}

func BindParamUUID(c echo.Context, key string) (*uuid.UUID, error) {
	param := c.Param(key)
	if param == "" {
		return nil, fmt.Errorf("parameter %s is required", key)
	}
	id, err := uuid.Parse(param)
	if err != nil {
		return nil, fmt.Errorf("parameter %s is not a valid UUID", key)
	}
	return &id, nil
}
