package problem

import (
	"net/http"

	"github.com/sazzer/newlanding/service/internal/response"
)

// New Problem Details response for an HTTP 404 Not Found.
func NewNotFoundProblem() response.Response {
	return response.New(Problem{
		Status: http.StatusNotFound,
	})
}
