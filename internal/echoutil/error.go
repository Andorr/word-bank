package echoutil

import (
	"net/http"

	"github.com/Andorr/word-bank/pkg/wordbank"
	"github.com/labstack/echo/v4"
)

// message defines the error message format.
type message struct {
	ErrorCode wordbank.ErrorCode `json:"errorCode,omitempty"`
	Error     string             `json:"error"`
}

func ErrBadRequest(code wordbank.ErrorCode, msg string) error {
	return HTTPError(http.StatusBadRequest, code, msg)
}

func ErrUnauthorized(msg string) error {
	return HTTPError(http.StatusUnauthorized, "", msg)
}

func ToHTTPError(err error, c echo.Context) error {
	werr, ok := err.(*wordbank.WordBankError)
	if !ok {
		return HTTPError(http.StatusInternalServerError, "", err.Error())
	}

	return HTTPError(werr.Status, werr.Code, err.Error())
}

func HTTPError(statusCode int, errCode wordbank.ErrorCode, msg string) error {
	return echo.NewHTTPError(statusCode, message{
		ErrorCode: errCode,
		Error:     msg,
	})
}
