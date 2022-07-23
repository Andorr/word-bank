package word

import (
	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/Andorr/word-bank/pkg/wordbank/models"
	"github.com/labstack/echo/v4"
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
		return err
	}

	query := c.QueryParam("query")
	words, werr := ctrl.WB.Word.QueryWords(ctx, models.WordQueryOptions{
		Query: &query,
	}, nil)
	if werr != nil {
		c.Logger().Errorf("failed to query words: %v", werr.Error())
		return err
	}

	c.JSON(200, words)
	return nil
}
