package auth0_test

import (
	"context"
	"testing"
	"time"

	"github.com/lestrrat-go/jwx/jwk"
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/authorization/auth0"
	"github.com/stretchr/testify/assert"
	"gopkg.in/h2non/gock.v1"
)

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseValidToken(t *testing.T) {
	private, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()

	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	securityContext, err := sut.ParseAccessToken(context.Background(), signed)
	assert.NoError(t, err)

	assert.Equal(t, authorization.Principal("google-oauth2|116440097717692497264"), securityContext.Principal)
	assert.Equal(t, issued, securityContext.IssuedAt)
	assert.Equal(t, expires, securityContext.ExpiresAt)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseTokenNoKey(t *testing.T) {
	private, _, _ := generateKey(t)
	_, _, keyset := generateKey(t)

	assert.NoError(t, private.Set(jwk.KeyIDKey, "anotherKey"))

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseTokenWrongKey(t *testing.T) {
	private, _, _ := generateKey(t)
	_, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseExpiredToken(t *testing.T) {
	private, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(-5 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseNotYetIssuedToken(t *testing.T) {
	private, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(5 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseTokenBadAudience(t *testing.T) {
	private, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://example.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"wrongAudience", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}

// Can't run Gock tests in parallel
// nolint:paralleltest
func TestParseTokenBadIssuer(t *testing.T) {
	private, _, keyset := generateKey(t)

	now := time.Now()
	issued := now.Add(-10 * time.Minute).UTC().Round(time.Second)
	expires := now.Add(10 * time.Minute).UTC().Round(time.Second)

	_, signed := generateToken(t, private, "https://other.xx.auth0.com/", "google-oauth2|116440097717692497264",
		"tag:newlanding,2021:auth0", issued, expires)

	defer gock.Off()
	keysMock := gock.New("https://example.xx.auth0.com").Get("/.well-known/jwks.json").Reply(200).JSON(keyset)

	sut := auth0.NewAccessTokenParser("https://example.xx.auth0.com", "tag:newlanding,2021:auth0")

	_, err := sut.ParseAccessToken(context.Background(), signed)
	assert.Equal(t, auth0.ErrParseToken, err)

	assert.True(t, keysMock.Done())
}
