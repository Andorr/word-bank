package word

import (
	"net/http"

	"github.com/Andorr/word-bank/pkg/echoutil"
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/labstack/echo/v4"
)

const (
	ErrCodeInvalidQueryOptions wordbank.ErrorCode = "INVALID_QUERY_OPTIONS"
)

type WordController struct {
	WB *wordbank.WordBank
}

func (c *WordController) Routes(e *echo.Echo) {
	g := e.Group("/api/v1/words")

	g.GET("/", c.QueryWords)

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

	words, werr := ctrl.WB.Word.QueryWords(ctx, *queryOptions, paginationOptions)
	if werr != nil {
		c.Logger().Errorf("failed to query words: %v", werr.Error())
		return echoutil.ToHTTPError(werr, c)
	}

	c.JSON(200, words)
	return nil
}

func (ctrl *WordController) CreateWord(c echo.Context) error {
	ctx, err := ctrl.WB.NewContext(c.Request().Context())
	if err != nil {
		c.Logger().Errorf("failed to create context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	word, err := echoutil.BindWord(c)
	if err != nil {
		c.Logger().Errorf("failed to bind word: %v", err.Error())
		return echoutil.ErrBadRequest(ErrCodeInvalidQueryOptions, err.Error())
	}

	werr := ctrl.WB.Word.InsertWord(ctx, word)
	if werr != nil {
		c.Logger().Errorf("failed to create word: %v", werr.Error())
		return echoutil.ToHTTPError(werr, c)
	}

	err = ctx.Commit()
	if err != nil {
		c.Logger().Errorf("failed to commit context: %v", err.Error())
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	c.JSON(200, word)
	return nil
}
