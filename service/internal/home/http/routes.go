package http

import (
	"net/http"

	"github.com/sazzer/newlanding/service/internal/server"
)

// Container for the HTTP routes.
type Routes struct{}

// Contribute the required HTTP routes for the home document.
func (r Routes) ContributeRoutes(router *server.Router) {
	router.Route(http.MethodGet, "/", r.index)
}
