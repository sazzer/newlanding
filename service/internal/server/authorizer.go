package server

import (
	"context"
	"net/http"
	"strings"

	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/problem"
)

// Unexported type to represent the key for the security landing in the context.
type contextKey string

// Attribute name for storing the Security Context.
const attributeSecurityContext = contextKey("NewLanding:SecurityContext")

// Create middleware to authenticate an incoming request if possible.
// This will not block requests that do not have an access token, but will only parse the access token if present
// and store it in the request. It will block the request if the access token is invalid however.
func authorizerMiddleware(authorizer authorization.Authorizer) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			header := r.Header.Get("authorization")

			if header == "" {
				next.ServeHTTP(w, r)

				return
			}

			if !strings.HasPrefix(header, "Bearer ") {
				log.Warn().Str("header", header).Msg("Authorization header is not a bearer token")

				response.New(problem.Unauthorized()).Send(w, r)
				return
			}

			token := strings.TrimPrefix(header, "Bearer ")

			sc, err := authorizer.ParseAccessToken(r.Context(), token)
			if err != nil {
				log.Warn().Str("header", header).Err(err).Msg("Unable to parse access token")

				response.New(problem.Unauthorized()).Send(w, r)
				return
			}

			log.Debug().Interface("securityContext", sc).Msg("Parsed Security Context")

			r = r.WithContext(context.WithValue(r.Context(), attributeSecurityContext, sc))

			next.ServeHTTP(w, r)
		})
	}
}
