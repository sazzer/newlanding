package http

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Container for the HTTP routes.
type Routes struct{}

// Contribute the required HTTP routes for the home document.
func (r Routes) ContributeRoutes(e *echo.Echo) {
	e.GET("/", server.WrapHandler(r.index))
}
