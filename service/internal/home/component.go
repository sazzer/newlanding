package home

import (
	"github.com/sazzer/newlanding/service/internal/home/http"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Component for the home document.
type Component struct {
	Routes server.RoutesContributor
}

// Create a new home document component.
func New() Component {
	routes := http.Routes{}

	return Component{
		Routes: routes,
	}
}
