package server

import (
	"github.com/labstack/echo/v4"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/response"
)

// Local wrapper around the Echo Context.
type Context struct {
	echo.Context
	SecurityContext *authorization.SecurityContext
}

// Local Handler function to handle incoming requests.
type HandlerFunc func(c Context) response.Response

// Wrapper around the Echo server to add routes.
type Router struct {
	e *echo.Echo
}

// Add a new route to the server.
func (r *Router) Route(method, url string, handler HandlerFunc) {
	r.e.Add(method, url, wrapHandler(handler))
}

// Wrap a handler function to make it work with the Echo handler function.
func wrapHandler(handler HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		sc := c.Get(attributeSecurityContext)
		securityContext, ok := sc.(authorization.SecurityContext)

		log.Info().Interface("sc", securityContext).Bool("ok", ok).Msg("SC")

		context := Context{
			Context:         c,
			SecurityContext: nil,
		}

		response := handler(context)

		return response.Send(c)
	}
}
