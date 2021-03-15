package users

import "time"

// Security Context to represent an authenticated user.
type SecurityContext struct {
	User      ID
	IssuedAt  time.Time
	ExpiresAt time.Time
}
