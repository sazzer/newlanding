package response_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/newlanding/service/internal/asserthttp"
	"github.com/sazzer/newlanding/service/internal/response"
)

func TestEmptyResponse(t *testing.T) {
	t.Parallel()

	response := response.New(map[string]interface{}{})

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

type statusCodePayload struct{}

func (p statusCodePayload) StatusCode() int {
	return http.StatusBadGateway
}

func TestStatusCodeResponse(t *testing.T) {
	t.Parallel()

	response := response.New(statusCodePayload{})

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

type contentTypePayload struct{}

func (p contentTypePayload) ContentType() string {
	return "text/plain"
}

func TestContentTypePayload(t *testing.T) {
	t.Parallel()

	response := response.New(contentTypePayload{})

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}
