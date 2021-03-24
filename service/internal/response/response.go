package response

import (
	"net/http"

	"github.com/rs/zerolog/log"
	"github.com/unrolled/render"
)

// Wrapper around an HTTP Response to send back to the client.
type Response struct {
	// The body of the response.
	body interface{}
	// The status code of the response.
	statusCode int
	// The content type of the response.
	contentType string
}

// Interface to allow response payloads to define the status code to use.
type WithStatusCode interface {
	// The status code to use.
	StatusCode() int
}

// Interface to allow response payloads to define the content type to use.
type WithContentType interface {
	// The content type to use.
	ContentType() string
}

// Create a new HTTP response for the given data.
func New(body interface{}) Response {
	response := Response{
		body:        body,
		statusCode:  http.StatusOK,
		contentType: "application/json",
	}

	if wsc, ok := body.(WithStatusCode); ok {
		response.statusCode = wsc.StatusCode()
	}

	if wct, ok := body.(WithContentType); ok {
		response.contentType = wct.ContentType()
	}

	return response
}

// Send the response to the client.
func (res Response) Send(w http.ResponseWriter, req *http.Request) {
	j := render.JSON{
		Head: render.Head{
			ContentType: res.contentType,
			Status:      res.statusCode,
		},
		Indent: true,
	}

	renderer := render.New()
	if err := renderer.Render(w, j, res.body); err != nil {
		log.Error().Err(err).Msg("Failed to send response")
	}
}
