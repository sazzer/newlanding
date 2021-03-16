package auth0

import (
	"context"
	"errors"

	"github.com/lestrrat-go/jwx/jwt"
	"github.com/rs/zerolog/log"
	"github.com/sazzer/newlanding/service/internal/users"
)

var ErrParseToken = errors.New("failed to parse access token")

// Means to parse an Access Token into a Security Context.
type AccessTokenParser struct {
	keys     Keyset
	issuer   string
	audience string
}

// Create a new Access Token Parser to parse tokens for the given domain.
func NewAccessTokenParser(domain Domain, audience string) AccessTokenParser {
	keys := NewKeyset(domain)

	return AccessTokenParser{
		keys:     keys,
		issuer:   domain.GetURL("/"),
		audience: audience,
	}
}

// Attempt to parse the provided access token into a security context.
func (a AccessTokenParser) ParseAccessToken(ctx context.Context, token string) (users.SecurityContext, error) {
	keyset, err := a.keys.FetchKeys(ctx)
	if err != nil {
		return users.SecurityContext{}, ErrFetchKeys
	}

	parsed, err := jwt.ParseString(token, jwt.WithKeySet(keyset))
	if err != nil {
		log.Warn().Err(err).Str("token", token).Msg("Failed to parse access token")

		return users.SecurityContext{}, ErrParseToken
	}

	err = jwt.Validate(parsed, jwt.WithAudience(a.audience), jwt.WithIssuer(a.issuer))
	if err != nil {
		log.Warn().Err(err).Str("token", token).Msg("Failed to validate access token")

		return users.SecurityContext{}, ErrParseToken
	}

	return users.SecurityContext{
		User:      users.ID(parsed.Subject()),
		IssuedAt:  parsed.IssuedAt().UTC(),
		ExpiresAt: parsed.Expiration().UTC(),
	}, nil
}
