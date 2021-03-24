package server

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/rs/zerolog/log"
)

// The HTTP Server.
type Server struct {
	port   uint16
	server *chi.Mux
}

// C0nstruct a new HTTP Server.
func New(port uint16) Server {
	log.Debug().Uint16("port", port).Msg("Building HTTP Server")

	r := chi.NewRouter()

	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Compress(5))
	r.Use(middleware.CleanPath)
	r.Use(middleware.StripSlashes)
	r.Use(cors.Handler(cors.Options{
		AllowCredentials: true,
	}))

	return Server{
		port:   port,
		server: r,
	}
}

// Start the HTTP Server.
func (s Server) Start() {
	address := fmt.Sprintf(":%d", s.port)

	log.Info().Str("address", address).Msg("Starting HTTP Server")

	if err := http.ListenAndServe(address, s.server); err != nil {
		log.Fatal().Err(err).Msg("Failed to start HTTP server")
	}
}
