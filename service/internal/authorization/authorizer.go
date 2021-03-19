package authorization

import "context"

// Authorizer to parse an access token into a security context.
type Authorizer interface {
	// Attempt to parse the provided access token into a security context.
	ParseAccessToken(ctx context.Context, token string) (SecurityContext, error)
}
