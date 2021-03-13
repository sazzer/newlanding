package http

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/hal"
)

// Route to serve up the home document.
func (r Routes) index(c echo.Context) error {
	model := Model{
		Name:    "newlanding",
		Version: "0.1.0",
	}
	model.WithLink("self", hal.Link{Href: "/"})

	return response.New(model).Send(c)
}
