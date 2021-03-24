package main

import (
	"github.com/kelseyhightower/envconfig"
	"github.com/rs/zerolog/log"
)

type config struct{}

func loadConfig() config {
	var c config

	if err := envconfig.Process("", &c); err != nil {
		log.Fatal().Err(err).Msg("Failed to load config from environment")
	}

	return c
}