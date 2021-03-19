package service

import (
	"github.com/sazzer/newlanding/service/internal/authorization"
	"github.com/sazzer/newlanding/service/internal/authorization/auth0"
)

// Component for authorization.
type AuthorizationComponent struct {
	Authorizer authorization.Authorizer
}

// Create a new instance of the authorization component.
func NewAuthorizationComponent(domain, audience string) AuthorizationComponent {
	authorizer := auth0.NewAccessTokenParser(auth0.Domain(domain), audience)

	return AuthorizationComponent{
		Authorizer: authorizer,
	}
}
