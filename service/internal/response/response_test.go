package response_test

import (
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/stretchr/testify/assert"
)

func TestEmptyResponse(t *testing.T) {
	t.Parallel()

	response := response.New(map[string]interface{}{})

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	assert.Equal(t, http.StatusOK, result.StatusCode)
	assert.Equal(t, "application/json", result.Header.Get("content-type"))

	body, err := ioutil.ReadAll(result.Body)
	assert.NoError(t, err)
	assert.Equal(t, []byte("{}\n"), body)
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

	assert.Equal(t, http.StatusBadGateway, result.StatusCode)
	assert.Equal(t, "application/json", result.Header.Get("content-type"))

	body, err := ioutil.ReadAll(result.Body)
	assert.NoError(t, err)
	assert.Equal(t, []byte("{}\n"), body)
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

	assert.Equal(t, http.StatusOK, result.StatusCode)
	assert.Equal(t, "text/plain", result.Header.Get("content-type"))

	body, err := ioutil.ReadAll(result.Body)
	assert.NoError(t, err)
	assert.Equal(t, []byte("{}\n"), body)
}
