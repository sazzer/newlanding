package service

import (
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/authorization/auth0"
)

// Component for authorization.
type authorizationComponent struct {
	Authorizer authorization.Authorizer
}

// Create a new instance of the authorization component.
func newAuthorizationComponent(domain, audience string) authorizationComponent {
	authorizer := auth0.NewAccessTokenParser(auth0.Domain(domain), audience)

	return authorizationComponent{
		Authorizer: authorizer,
	}
}
