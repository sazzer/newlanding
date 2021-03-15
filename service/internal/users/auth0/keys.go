package auth0

import (
	"context"
	"errors"

	"github.com/lestrrat-go/jwx/jwk"
)

// ErrFetchKeys is an error returned when fetching of JWK Keys fails.
var ErrFetchKeys = errors.New("failed to fetch JWK keys")

// Wrapper around the Auth0 Keyset to get the correct key for the token.
type Keyset struct {
	keys *jwk.AutoRefresh
	url  string
}

func NewKeyset(domain Domain) Keyset {
	url := domain.GetURL("/.well-known/jwks.json")
	keys := jwk.NewAutoRefresh(context.Background())
	keys.Configure(url)

	return Keyset{
		keys: keys,
		url:  url,
	}
}

// Get the JWK Set to use for decoding tokens.
//
// TODO: This doesn't have any refresh support right now.
func (k Keyset) FetchKeys(ctx context.Context) (jwk.Set, error) {
	set, err := k.keys.Fetch(ctx, k.url)
	if err != nil {
		return nil, ErrFetchKeys
	}

	return set, nil
}
