package server

import (
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/sazzer/newlanding/service/internal/response"
)

// Context object that is handled to handler funcs that contain request details.
type RequestContext struct {
	// The actual HTTP request.
	*http.Request
}

// Handler function that handles an HTTP request and returns some response.
type HandlerFunc func(req RequestContext) response.Response

// Wrapper around the Chi Router.
type Router struct {
	mux *chi.Mux
}

// Register a new GET handler for the given URI.
func (r Router) GET(uri string, handler HandlerFunc) {
	r.mux.Get(uri, r.wrapHandler(handler))
}

// Wrap a New Landing HTTP Handler to make it compatible with Chi.
func (r Router) wrapHandler(handler HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, req *http.Request) {
		context := RequestContext{
			Request: req,
		}

		response := handler(context)

		response.Send(w, req)
	}
}
