package service

import (
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/home"
	"github.com/sazzer/newlanding/service/internal/server"
)

// The actual New Landing service.
type Service struct {
	server server.Server
}

// Create a new instance of the service that's ready to run.
func New(cfg Config) Service {
	log.Info().Msg("Building New Landing")

	home := home.New()

	server := server.New(cfg.Port, []server.RoutesContributor{
		home.Routes,
	})

	log.Info().Msg("Built New Landing")

	return Service{
		server: server.Server,
	}
}

func (s Service) Start() {
	log.Info().Msg("Starting New Landing")
	s.server.Start()
}
