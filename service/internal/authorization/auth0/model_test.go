package auth0_test

import (
	"testing"

	"github.com/sazzer/newlanding/service/internal/authorization/auth0"
	"github.com/stretchr/testify/assert"
)

func TestGetURL(t *testing.T) {
	t.Parallel()

	d := auth0.Domain("https://example.xx.auth0.com")
	url := d.GetURL("/.well-known/jwks.json")

	assert.Equal(t, "https://example.xx.auth0.com/.well-known/jwks.json", url)
}
