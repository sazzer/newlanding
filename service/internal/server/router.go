package server

import (
	"net/http"
	"strings"

	"github.com/go-chi/chi/v5"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/response"
)

// Local wrapper around the HTTP Request.
type Context struct {
	Req             *http.Request
	SecurityContext *authorization.SecurityContext
}

// Local Handler function to handle incoming requests.
type HandlerFunc func(c Context) response.Response

// Wrapper around the Echo server to add routes.
type Router struct {
	r *chi.Mux
}

func NewRouter(r *chi.Mux) Router {
	return Router{
		r: r,
	}
}

// Add a new route to the server.
func (r *Router) Route(method, url string, handler HandlerFunc) {
	wrapper := r.wrapHandler(handler)

	switch strings.ToUpper(method) {
	case "GET":
		r.r.Get(url, wrapper)
	default:
		log.Fatal().Str("method", method).Str("url", url).Msg("Unsupported HTTP method")
	}
}

// Wrap a handler function to make it work with the Echo handler function.
func (r *Router) wrapHandler(handler HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx := Context{
			Req: r,
		}

		sc := r.Context().Value(attributeSecurityContext)
		if securityContext, ok := sc.(authorization.SecurityContext); ok {
			ctx.SecurityContext = &securityContext
		}

		res := handler(ctx)

		res.Send(w, r)
	}
}
