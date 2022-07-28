package word

import "github.com/labstack/echo/v4"

func (c *WordController) Routes(e *echo.Echo) {
	g := e.Group("/api/v1/words")

	g.GET("/", c.QueryWords)
	g.POST("/", c.CreateWord)
	g.GET("/:id", c.GetWord)
	g.PUT("/:id", c.UpdateWord)
	g.DELETE("/:id", c.DeleteWord)
	g.GET("/random", c.RandomWord)

	g2 := e.Group("/api/v1/folders")

	g2.GET("/", c.QueryFolders)
	g2.POST("/", c.CreateFolder)
	g2.GET("/:id", c.GetFolder)
	g2.PUT("/:id", c.UpdateFolder)
	g2.DELETE("/:id", c.DeleteFolder)
}
