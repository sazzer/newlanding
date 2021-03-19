package authorization

import (
	"time"

	"github.com/sazzer/newlanding/service/internal/response/hal"
)

// Representation of the principal of a security context.
type Principal string

// Security Context to represent an authenticated user.
type SecurityContext struct {
	Principal Principal
	IssuedAt  time.Time
	ExpiresAt time.Time
}

// Produce a HAL Link for this user.
func (p Principal) ToLink() hal.Link {
	return hal.NewTemplateLink("/users/{id}", map[string]interface{}{"id": p})
}
