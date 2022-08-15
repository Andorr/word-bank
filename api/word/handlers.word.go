package word

import (
	"net/http"

	"github.com/Andorr/word-bank/internal/echoutil"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/google/uuid"
	"github.com/labstack/echo/v4"
)

const (
	ErrCodeInvalidQueryOptions wordbank.ErrorCode = "INVALID_QUERY_OPTIONS"
	ErrCodeInvalidBody         wordbank.ErrorCode = "INVALID_BODY"
	ErrCodeInvalidUUID         wordbank.ErrorCode = "INVALID_UUID"
)

type WordController struct {
	WB *wordbank.WordBank
}

func (ctrl *WordController) QueryWords(c echo.Context) error {
	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	queryOptions, err := echoutil.BindWordQueryOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind query options: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidQueryOptions, err.Error())
	}

	paginationOptions, err := echoutil.BindPaginationOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind pagination options: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidQueryOptions, err.Error())
	}

	words, err := ctrl.WB.Word.QueryWords(ctx, *queryOptions, paginationOptions)
	if err != nil {
		c.Logger().Errorf("failed to query words: %v", err.Error())
		return echoutil.ToHTTPError(err, c)
	}

	c.JSON(200, words)
	return nil
}

func (ctrl *WordController) CreateWord(c echo.Context) error {

	type request struct {
		models.Word
		Folder *uuid.UUID `json:"folder"`
	}
	var req request
	if err := c.Bind(&req); err != nil {
		c.Logger().Errorf("failed to bind request: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidBody, err.Error())
	}

	return ctrl.WB.RunTx(func(ctx *wordbank.WordBankContext) error {
		err := ctrl.WB.Word.InsertWord(ctx, &req.Word)
		if err != nil {
			c.Logger().Errorf("failed to create word: %v", err.Error())
			return echoutil.ToHTTPError(err, c)
		}

		if req.Folder != nil {
			_, err = ctrl.WB.Word.UpdateFolder(ctx, models.FolderUpdateOptions{
				ID:  *req.Folder,
				Add: []uuid.UUID{*req.Word.ID},
			})
			if err != nil {
				c.Logger().Errorf("failed to add word to folder: %v", err.Error())
				return echoutil.ToHTTPError(err, c)
			}
		}

		c.JSON(200, req.Word)
		return nil
	})

}

func (ctrl *WordController) UpdateWord(c echo.Context) error {

	options, err := echoutil.BindWordUpdateOptions(c)
	if err != nil {
		c.Logger().Errorf("failed to bind update options: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidQueryOptions, err.Error())
	}

	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	word, err := ctrl.WB.Word.UpdateWord(ctx, *options)
	if err != nil {
		c.Logger().Errorf("failed to update word: %v", err.Error())
		return echoutil.ToHTTPError(err, c)
	}

	c.JSON(200, word)
	return nil
}

func (ctrl *WordController) DeleteWord(c echo.Context) error {
	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	id, err := echoutil.BindParamUUID(c, "id")
	if err != nil {
		c.Logger().Errorf("failed to bind word: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidUUID, err.Error())
	}

	err = ctrl.WB.Word.DeleteWord(ctx, *id)
	if err != nil {
		c.Logger().Errorf("failed to delete word: %v", err.Error())
		return echoutil.ToHTTPError(err, c)
	}

	return nil
}

func (ctrl *WordController) GetWord(c echo.Context) error {
	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	id, err := echoutil.BindParamUUID(c, "id")
	if err != nil {
		c.Logger().Errorf("failed to bind word: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidUUID, err.Error())
	}

	word, err := ctrl.WB.Word.GetWord(ctx, *id)
	if err != nil {
		c.Logger().Errorf("failed to get word: %v", err.Error())
		return echoutil.ToHTTPError(err, c)
	}

	c.JSON(200, word)
	return nil
}

func (ctrl *WordController) RandomWord(c echo.Context) error {
	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	type request struct {
		Count int `json:"count"`
	}
	var req request
	if err := c.Bind(&req); err != nil {
		c.Logger().Errorf("failed to bind request: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidBody, err.Error())
	}

	word, err := ctrl.WB.Word.RandomWords(ctx, req.Count)
	if err != nil {
		c.Logger().Errorf("failed to random word: %v", err.Error())
		return echoutil.ToHTTPError(err, c)
	}

	c.JSON(200, word)
	return nil
}
