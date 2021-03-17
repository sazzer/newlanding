package auth0_test

import (
	"context"
	"testing"

	"github.com/sazzer/newlanding/service/internal/authorization/auth0"
	"github.com/stretchr/testify/assert"
	"gopkg.in/h2non/gock.v1"
)

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestGetKnownKey(t *testing.T) {
	_, publicKey, keyset := generateKey(t)

	defer gock.Off()
	mock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewKeyset("https://example.xx.auth0.com")

	keys, err := sut.FetchKeys(context.Background())
	assert.NoError(t, err)

	assert.Equal(t, 1, keys.Len())

	key, ok := keys.LookupKeyID("myKeyID")
	assert.True(t, ok)
	assert.Equal(t, publicKey, key)

	assert.True(t, mock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestGetUnknownKey(t *testing.T) {
	defer gock.Off()
	mock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(404)

	sut := auth0.NewKeyset("https://example.xx.auth0.com")

	_, err := sut.FetchKeys(context.Background())
	assert.Equal(t, auth0.ErrFetchKeys, err)

	assert.True(t, mock.Done())
}
