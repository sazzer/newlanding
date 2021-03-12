package server

import (
	"fmt"

	"github.com/rs/zerolog/log"
)

// Wrapper around the HTTP server.
type Server struct {
	port uint16
}

// Create a new instance of the HTTP server.
func New(port uint16) Server {
	return Server{port}
}

// Start the HTTP Server listening.
func (s Server) Start() {
	address := fmt.Sprintf(":%d", s.port)

	log.Info().Str("address", address).Msg("Starting HTTP Server")
}
