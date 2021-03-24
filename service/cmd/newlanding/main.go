package main

import (
	"os"
	"time"

	_ "github.com/joho/godotenv/autoload"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/rs/zerolog/pkgerrors"
	"github.com/sazzer/newlanding/service/internal/service"
)

func main() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	zerolog.SetGlobalLevel(zerolog.DebugLevel)
	zerolog.ErrorStackMarshaler = pkgerrors.MarshalStack

	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout, TimeFormat: time.RFC3339}).With().Caller().Logger()

	config := loadConfig()

	log.Info().Interface("config", config).Msg("Config")

	service := service.New(service.Config{
		HTTP: service.HTTPConfig{
			Port: config.Port,
		},
	})

	service.Start()
}
