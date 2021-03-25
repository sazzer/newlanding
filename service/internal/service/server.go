package service

import "github.com/sazzer/newlanding/service/internal/server"

// Builder for building the server component.
type serverComponentBuilder struct {
	port   uint16
	routes []server.RouteContributor
}

// Create a new server component.
func newServer(port uint16) serverComponentBuilder {
	return serverComponentBuilder{
		port:   port,
		routes: nil,
	}
}

// Register a new set of routes with the server.
func (s *serverComponentBuilder) withRoutes(r server.RouteContributor) *serverComponentBuilder {
	s.routes = append(s.routes, r)
	return s
}

// Build the server.
func (s serverComponentBuilder) build() server.Server {
	return server.New(s.port, s.routes)
}
