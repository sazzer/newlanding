package http

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func (r Routes) index(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}
