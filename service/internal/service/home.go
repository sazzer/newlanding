package service

import (
	"github.com/sazzer/newlanding/service/internal/home/http"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Component for the home document.
type homeComponent struct {
	Routes server.RoutesContributor
}

// Create a new home document component.
func newHomeComponent() homeComponent {
	routes := http.Routes{}

	return homeComponent{
		Routes: routes,
	}
}
