package auth0_test

import (
	"crypto/rand"
	"crypto/rsa"
	"testing"
	"time"

	"github.com/lestrrat-go/jwx/jwa"
	"github.com/lestrrat-go/jwx/jwk"
	"github.com/lestrrat-go/jwx/jwt"
	"github.com/stretchr/testify/assert"
)

func generateKey(t *testing.T) (jwk.Key, jwk.Key, jwk.Set) {
	t.Helper()

	raw, err := rsa.GenerateKey(rand.Reader, 2048)
	assert.NoError(t, err)

	private, err := jwk.New(raw)
	assert.NoError(t, err)
	assert.Implements(t, (*jwk.RSAPrivateKey)(nil), private)
	assert.NoError(t, private.Set(jwk.KeyIDKey, "myKeyID"))
	assert.NoError(t, private.Set(jwk.AlgorithmKey, "RSA256"))

	public, err := jwk.New(raw.PublicKey)
	assert.NoError(t, err)
	assert.Implements(t, (*jwk.RSAPublicKey)(nil), public)
	assert.NoError(t, public.Set(jwk.KeyIDKey, "myKeyID"))
	assert.NoError(t, public.Set(jwk.AlgorithmKey, "RSA256"))

	set := jwk.NewSet()
	set.Add(public)

	return private, public, set
}

// nolint:unparam
func generateToken(t *testing.T, privateKey jwk.Key, issuer, subject, audience string, issued, expired time.Time) (jwt.Token, string) {
	t.Helper()

	token := jwt.New()
	assert.NoError(t, token.Set(jwt.IssuerKey, issuer))
	assert.NoError(t, token.Set(jwt.SubjectKey, subject))
	assert.NoError(t, token.Set(jwt.AudienceKey, []string{audience}))
	assert.NoError(t, token.Set(jwt.ExpirationKey, expired))
	assert.NoError(t, token.Set(jwt.IssuedAtKey, issued))

	signed, err := jwt.Sign(token, jwa.RS256, privateKey)
	assert.NoError(t, err)

	return token, string(signed)
}
