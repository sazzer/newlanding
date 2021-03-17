package authorization

import "time"

type Principal string

// Security Context to represent an authenticated user.
type SecurityContext struct {
	Principal Principal
	IssuedAt  time.Time
	ExpiresAt time.Time
}
