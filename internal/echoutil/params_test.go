package echoutil

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/assert"
)

func TestBindPaginationOptionsNormal(t *testing.T) {
	e := echo.New()

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	values := req.URL.Query()
	values.Add("page", "1")
	values.Add("limit", "10")
	req.URL.RawQuery = values.Encode()
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)

	paginationOptions, err := BindPaginationOptions(c)
	if err != nil {
		t.Errorf("Error in BindPaginationOptions: %s", err)
		t.FailNow()
	}

	assert.Equal(t, 1, paginationOptions.Page)
	assert.Equal(t, 10, paginationOptions.Limit)

	req = httptest.NewRequest(http.MethodGet, "/", nil)
	rec = httptest.NewRecorder()
	values = req.URL.Query()
	values.Set("page", "not-int")
	req.URL.RawQuery = values.Encode()
	c = e.NewContext(req, rec)

	paginationOptions, err = BindPaginationOptions(c)
	assert.Error(t, err)

}
