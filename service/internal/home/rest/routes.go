package rest

import "github.com/sazzer/newlanding/service/internal/server"

// Routes for the Home Document.
type routes struct{}

// Create the routes for the home document.
func New() server.RouteContributor {
	return routes{}
}

func (r routes) ContributeRoutes(router server.Router) {
	router.GET("/", r.index)
}
