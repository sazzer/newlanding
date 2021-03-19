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
	model.WithLink("self", hal.NewLink("/"))

	if c.SecurityContext != nil {
		model.WithLink("user", c.SecurityContext.Principal.ToLink())
	}

	return response.New(model)
}
