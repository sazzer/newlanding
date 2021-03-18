package service

import "github.com/sazzer/newlanding/service/internal/server"

// Component to represent the HTTP server.
type ServerComponent struct {
	Server server.Server
}

// Create a new instance of the HTTP Server component.
func NewServerComponent(port uint16, routes []server.RoutesContributor) ServerComponent {
	server := server.New(port, routes)

	return ServerComponent{
		Server: server,
	}
}
