package main

import (
	"github.com/kelseyhightower/envconfig"
	"github.com/rs/zerolog/log"
)

type config struct {
	Port          uint16 `default:"8000"`
	Auth0Domain   string `envconfig:"AUTH0_DOMAIN" required:"true"`
	Auth0Audience string `envconfig:"AUTH0_AUDIENCE" required:"true"`
}

func loadConfig() config {
	var c config

	if err := envconfig.Process("", &c); err != nil {
		log.Fatal().Err(err).Msg("Failed to load config from environment")
	}

	return c
}
