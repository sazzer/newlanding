package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/server"
)

// The actual New Landing service.
type Service struct {
	server server.Server
}

// Create a new instance of the service that's ready to run.
func New(cfg Config) Service {
	log.Info().Msg("Building New Landing")

	_ = NewAuthorizationComponent(cfg.Auth0.Domain, cfg.Auth0.Audience)
	home := NewHomeComponent()

	server := NewServerComponent(cfg.Port, []server.RoutesContributor{
		home.Routes,
	})

	log.Info().Msg("Built New Landing")

	return Service{
		server: server.Server,
	}
}

// Start the service running.
func (s Service) Start() {
	log.Info().Msg("Starting New Landing")
	s.server.Start()
}
