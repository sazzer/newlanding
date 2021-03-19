package service

import "github.com/sazzer/newlanding/service/internal/server"

// Component to represent the HTTP server.
type serverComponent struct {
	Server server.Server
}

// Create a new instance of the HTTP Server component.
func newServerComponent(port uint16, authorization authorizationComponent, routes []server.RoutesContributor) serverComponent {
	server := server.New(port, authorization.Authorizer, routes)

	return serverComponent{
		Server: server,
	}
}
