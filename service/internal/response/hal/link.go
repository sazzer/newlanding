package hal

import (
	"github.com/jtacoma/uritemplates"
	"github.com/rs/zerolog/log"
)

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
func NewNamedLink(name, href string) Link {
	return Link{
		Href: href,
		Name: name,
	}
}

// Create a new, unnamed link.
func NewTemplateLink(href string, binds map[string]interface{}) Link {
	template, err := uritemplates.Parse(href)
	if err != nil {
		log.Fatal().Err(err).Str("href", href).Msg("Failed to parse URI Template")
	}

	expanded, err := template.Expand(binds)
	if err != nil {
		log.Fatal().Err(err).Str("href", href).Msg("Failed to expand URI Template")
	}

	return Link{
		Href: expanded,
		Name: "",
	}
}

// Create a new, named link.
func NewNamedTemplateLink(name, href string, binds map[string]interface{}) Link {
	template, err := uritemplates.Parse(href)
	if err != nil {
		log.Fatal().Err(err).Str("href", href).Msg("Failed to parse URI Template")
	}

	expanded, err := template.Expand(binds)
	if err != nil {
		log.Fatal().Err(err).Str("href", href).Msg("Failed to expand URI Template")
	}

	return Link{
		Href: expanded,
		Name: name,
	}
}
