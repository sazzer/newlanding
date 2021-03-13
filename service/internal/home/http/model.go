package http

import "github.com/sazzer/newlanding/service/internal/response/hal"

// Model representation of the home document.
type Model struct {
	hal.Document
	Name    string `json:"name"`
	Version string `json:"version"`
}
