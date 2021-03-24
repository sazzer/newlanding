package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/server"
)

// The actual service itself.
type Service struct {
	server server.Server
}

// Construct a new instance of the service.
func New(config Config) Service {
	log.Info().Msg("Building New Landing")

	server := server.New(config.HTTP.Port)

	log.Info().Msg("Built New Landing")

	return Service{server}
}

// Start the service running.
func (s Service) Start() {
	log.Info().Msg("Starting New Landing")

	s.server.Start()
}
