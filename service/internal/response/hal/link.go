package hal

// Representation of a single link within a HAL document.
type Link struct {
	Href string `json:"href"`
	Name string `json:"name,omitempty"`
}

// Create a new, unnamed link.
func NewLink(href string) Link {
	return Link{
		Href: href,
		Name: "",
	}
}

// Create a new, named link.
func NewNamedLink(href, name string) Link {
	return Link{
		Href: href,
		Name: name,
	}
}
