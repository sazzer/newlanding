package response

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

// Representation of the response to an HTTP Request.
type Response struct {
	// The body of the HTTP response.
	body interface{}
	// The status code for the HTTP response.
	status int
	// Any headers to include in the HTTP response.
	headers map[string]string
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
	statusCode := http.StatusOK
	if wsc, ok := body.(WithStatusCode); ok {
		statusCode = wsc.StatusCode()
	}

	headers := map[string]string{}
	if wct, ok := body.(WithContentType); ok {
		headers["content-type"] = wct.ContentType()
	}

	return Response{
		body:    body,
		status:  statusCode,
		headers: headers,
	}
}

// Actually send the response to the client.
func (r Response) Send(c echo.Context) error {
	for name, value := range r.headers {
		c.Response().Header().Set(name, value)
	}

	return c.JSON(r.status, r.body)
}
