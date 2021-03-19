package response

import (
	"net/http"

	"github.com/rs/zerolog/log"
	"github.com/unrolled/render"
)

// Representation of the response to an HTTP Request.
type Response struct {
	// The body of the HTTP response.
	body interface{}
	// The status code for the HTTP response.
	status int
	// The content type for the HTTP response.
	contentType string
}

// Interface that can be implemented by the response data to indicate it's status code.
type WithStatusCode interface {
	// Generate the status code of the response.
	StatusCode() int
}

// Interface that can be implemented by the response data to indicate it's content type.
type WithContentType interface {
	// Generate the content type of the response.
	ContentType() string
}

// Create a new Response for the provided payload data.
// If this payload happens to also implement the WithStatusCode or WithContentType interfaces
// then these will be taken into account, otherwise the defaults will be used.
func New(body interface{}) Response {
	response := Response{
		body:        body,
		status:      http.StatusOK,
		contentType: "application/json",
	}

	if wsc, ok := body.(WithStatusCode); ok {
		response.status = wsc.StatusCode()
	}

	if wct, ok := body.(WithContentType); ok {
		response.contentType = wct.ContentType()
	}

	return response
}

// Actually send the response to the client.
func (res Response) Send(w http.ResponseWriter, req *http.Request) {
	j := render.JSON{
		Head: render.Head{
			ContentType: res.contentType,
			Status:      res.status,
		},
		Indent: true,
	}

	renderer := render.New()
	if err := renderer.Render(w, j, res.body); err != nil {
		log.Error().Err(err).Msg("Failed to send response")
	}
}
