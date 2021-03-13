package server

// Component to represent the HTTP server.
type Component struct {
	Server Server
}

// Create a new instance of the HTTP Server component.
func New(port uint16, routes []RoutesContributor) Component {
	server := newServer(port, routes)

	return Component{
		Server: server,
	}
}
