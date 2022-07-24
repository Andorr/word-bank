package middleware

import (
	"strings"

	"github.com/Andorr/word-bank/pkg/echoutil"
	"github.com/labstack/echo/v4"
)

func AuthWithSingleToken(token string) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			authHeader := c.Request().Header.Get("Authorization")
			components := strings.Split(authHeader, " ")
			if len(components) != 2 {
				return echoutil.ErrUnauthorized("invalid authorization header")
			}
			if components[0] != "Bearer" {
				return echoutil.ErrUnauthorized("invalid authorization header")
			}
			if components[1] != token {
				return echoutil.ErrUnauthorized("invalid authorization header")
			}
			return next(c)
		}
	}
}
