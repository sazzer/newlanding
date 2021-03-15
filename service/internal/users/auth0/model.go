package auth0

import "fmt"

// Representation of the Auth0 Domain that we are authorizing against.
type Domain string

// Build a URL underneath the domain for use in other clients.
func (d Domain) GetURL(url string) string {
	return fmt.Sprintf("%s%s", d, url)
}
