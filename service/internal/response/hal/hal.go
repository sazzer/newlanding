package hal

import "github.com/rs/zerolog/log"

// Representation of a single link in a HAL document.
type Link struct {
	// The href of the link.
	Href string `json:"href"`
	// The name of the link.
	Name string `json:"name,omitempty"`
}

// Base data for a HAL document.
type Hal struct {
	// The links in the document.
	Links map[string]interface{} `json:"_links,omitempty"`
}

func (h Hal) ContentType() string {
	return "application/hal+json"
}

func (h *Hal) WithLink(rel string, newLink Link) *Hal {
	if h.Links == nil {
		h.Links = map[string]interface{}{}
	}

	if links, ok := h.Links[rel]; ok {
		if link, ok := links.(Link); ok {
			h.Links[rel] = []Link{link, newLink}
		} else if oldLinks, ok := links.([]Link); ok {
			h.Links[rel] = append(oldLinks, newLink)
		} else {
			log.Error().Str("rel", rel).Interface("link", newLink).Interface("oldLinks", links).Msg("Existing links not of supported type")
		}
	} else {
		h.Links[rel] = newLink
	}

	return h
}
