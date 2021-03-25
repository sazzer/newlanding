package hal_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/sazzer/newlanding/service/internal/asserthttp"
	"github.com/sazzer/newlanding/service/internal/response"
	"github.com/sazzer/newlanding/service/internal/response/hal"
)

func TestEmptyDocument(t *testing.T) {
	t.Parallel()

	document := hal.Hal{}
	response := response.New(document)

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

func TestSingleLink(t *testing.T) {
	t.Parallel()

	document := hal.Hal{}
	document.WithLink("self", hal.Link{Href: "/abc"})
	response := response.New(document)

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

func TestTwoSingleLinks(t *testing.T) {
	t.Parallel()

	document := hal.Hal{}
	document.WithLink("self", hal.Link{Href: "/abc"})
	document.WithLink("other", hal.Link{Href: "/def", Name: "Other"})
	response := response.New(document)

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

func TestRepeatedLinks(t *testing.T) {
	t.Parallel()

	document := hal.Hal{}
	document.WithLink("item", hal.Link{Href: "/abc"})
	document.WithLink("item", hal.Link{Href: "/def", Name: "Other"})
	response := response.New(document)

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}

func TestSingleAndRepeatedLinks(t *testing.T) {
	t.Parallel()

	document := hal.Hal{}
	document.WithLink("self", hal.Link{Href: "/self"})
	document.WithLink("item", hal.Link{Href: "/abc"})
	document.WithLink("item", hal.Link{Href: "/def", Name: "Other"})
	response := response.New(document)

	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	response.Send(rec, req)

	result := rec.Result()
	defer result.Body.Close()

	asserthttp.AssertResponse(t, result)
}
