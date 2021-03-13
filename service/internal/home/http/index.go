package http

import (
	"github.com/labstack/echo/v4"
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/hal"
)

func (r Routes) index(c echo.Context) error {
	model := Model{}
	model.WithLink("self", hal.Link{Href: "/"})

	return response.New(model).Send(c)
}
