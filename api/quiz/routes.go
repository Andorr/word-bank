package quiz

import "github.com/labstack/echo/v4"

func (c *QuizController) Routes(e *echo.Echo) {
	g := e.Group("/api/v1/quiz")

	g.POST("/", c.InitQuiz)
	g.POST("/result", c.InsertQuizResult)
}
