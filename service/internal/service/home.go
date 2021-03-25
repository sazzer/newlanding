package service

import (
	"github.com/sazzer/newlanding/service/internal/home/rest"
	"github.com/sazzer/newlanding/service/internal/server"
)

// Component for the home document.
type homeComponent struct {
	routes server.RouteContributor
}

// Create the home document component.
func newHome() homeComponent {
	return homeComponent{
		routes: rest.New(),
	}
}
