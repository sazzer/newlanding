package problem_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/newlanding/service/internal/asserthttp"
	"github.com/sazzer/newlanding/service/internal/response/problem"
)

func TestNotFound(t *testing.T) {
	t.Parallel()

	problem := problem.NewNotFoundProblem()

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	problem.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}
