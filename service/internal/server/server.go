package server

import (
	"fmt"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/rs/zerolog/log"
)

// Interface that components can implement if they are able to contribute routes to the server.
type RoutesContributor interface {
	// Contribute some routes to the HTTP Server.
	ContributeRoutes(r *Router)
}

// Wrapper around the HTTP server.
type Server struct {
	port   uint16
	server *echo.Echo
}

// Create a new instance of the HTTP server.
func newServer(port uint16, routes []RoutesContributor) Server {
	e := echo.New()

	e.Use(middleware.RequestID())
	e.Pre(middleware.RemoveTrailingSlash())
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowCredentials: true,
	}))
	e.Use(middleware.Decompress())
	e.Use(middleware.Gzip())

	router := Router{e}

	for _, r := range routes {
		r.ContributeRoutes(&router)
	}

	return Server{
		port:   port,
		server: e,
	}
}

// Start the HTTP Server listening.
func (s Server) Start() {
	address := fmt.Sprintf(":%d", s.port)

	log.Info().Str("address", address).Msg("Starting HTTP Server")

	if err := s.server.Start(address); err != nil {
		log.Fatal().Err(err).Msg("Failed to start HTTP server")
	}
}
