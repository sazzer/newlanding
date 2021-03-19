package service

// Configuration required to build the service.
type Config struct {
	// The port to run the HTTP server on.
	Port uint16

	// The details for working with Auth0.
	Auth0 Auth0Config
}

// Configuration for working with Auth0.
type Auth0Config struct {
	// The Auth0 domain.
	Domain string
	// The Auth0 API audience.
	Audience string
}
