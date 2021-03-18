package service

import (
	"github.com/sazzer/newlanding/service/internal/home/http"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Component for the home document.
type HomeComponent struct {
	Routes server.RoutesContributor
}

// Create a new home document component.
func NewHomeComponent() HomeComponent {
	routes := http.Routes{}

	return HomeComponent{
		Routes: routes,
	}
}
