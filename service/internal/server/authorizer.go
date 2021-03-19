package server

import (
	"net/http"
	"strings"

	"github.com/labstack/echo/v4"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/authorization"
)

// Attribute name for storing the Security Context.
const attributeSecurityContext = "NewLanding:SecurityContext"

// Create middleware to authenticate an incoming request if possible.
// This will not block requests that do not have an access token, but will only parse the access token if present
// and store it in the request. It will block the request if the access token is invalid however.
func authorizerMiddleware(authorizer authorization.Authorizer) echo.MiddlewareFunc {
	return func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			header := c.Request().Header.Get("authorization")

			if header == "" {
				return next(c)
			}

			if !strings.HasPrefix(header, "Bearer ") {
				log.Warn().Str("header", header).Msg("Authorization header is not a bearer token")

				return c.JSON(http.StatusForbidden, "")
			}

			token := strings.TrimPrefix(header, "Bearer ")

			sc, err := authorizer.ParseAccessToken(c.Request().Context(), token)
			if err != nil {
				log.Warn().Str("header", header).Err(err).Msg("Unable to parse access token")

				return c.JSON(http.StatusForbidden, "")
			}

			log.Debug().Interface("securityContext", sc).Msg("Parsed Security Context")

			c.Set(attributeSecurityContext, sc)

			return next(c)
		}
	}
}
