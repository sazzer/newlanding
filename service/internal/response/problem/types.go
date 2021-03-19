package problem

import "net/http"

// Construct a Problem for an Unauthorized response.
func Unauthorized() Problem {
	return Problem{
		Status: http.StatusUnauthorized,
	}
}
