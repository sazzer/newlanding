package server

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/authorization"
)

// Interface that components can implement if they are able to contribute routes to the server.
type RoutesContributor interface {
	// Contribute some routes to the HTTP Server.
	ContributeRoutes(r *Router)
}

// Wrapper around the HTTP server.
type Server struct {
	port   uint16
	server *chi.Mux
}

// Create a new instance of the HTTP server.
func New(port uint16, authorizer authorization.Authorizer, routes []RoutesContributor) Server {
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
	r.Use(authorizerMiddleware(authorizer))

	router := NewRouter(r)

	for _, route := range routes {
		route.ContributeRoutes(&router)
	}

	return Server{
		port:   port,
		server: r,
	}
}

// Start the HTTP Server listening.
func (s Server) Start() {
	address := fmt.Sprintf(":%d", s.port)

	log.Info().Str("address", address).Msg("Starting HTTP Server")

	if err := http.ListenAndServe(address, s.server); err != nil {
		log.Fatal().Err(err).Msg("Failed to start HTTP server")
	}
}
