package rest

import (
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/hal"
	"github.com/sazzer/newlanding/service/internal/server"
)

func (r routes) index(req server.RequestContext) response.Response {
	h := hal.Hal{}
	h.WithLink("self", hal.Link{Href: "/"})

	return response.New(h)
}
