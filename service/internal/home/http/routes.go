package http

import "github.com/labstack/echo/v4"

type Routes struct{}

func (r Routes) ContributeRoutes(e *echo.Echo) {
	e.GET("/", r.index)
}
