package http

import (
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/hal"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Route to serve up the home document.
func (r Routes) index(c server.Context) response.Response {
	model := Model{
		Name:    "newlanding",
		Version: "0.1.0",
	}
	model.WithLink("self", hal.Link{Href: "/"})

	return response.New(model)
}
