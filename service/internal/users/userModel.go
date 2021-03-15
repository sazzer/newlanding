package users

import "time"

// Type representing the ID of a User.
type ID string

// Data representing a user.
type Data struct {
	Name           string
	Email          string
	EmailVerified  bool
	SocialProvider string
}

// Identity of a user.
type Identity struct {
	ID      ID
	Version string
	Created time.Time
	Updated time.Time
}

// A persisted user.
type User struct {
	Identity
	Data
}
