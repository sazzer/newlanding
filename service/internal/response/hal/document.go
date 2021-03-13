package hal

import "github.com/rs/zerolog/log"

// Representation of a HAL document.
type Document struct {
	Links map[string]interface{} `json:"_links,omitempty"`
}

// Add a new link to the document.
func (d *Document) WithLink(rel string, link Link) *Document {
	if d.Links == nil {
		d.Links = map[string]interface{}{}
	}

	if val, ok := d.Links[rel]; ok {
		if links, ok := val.(Link); ok {
			d.Links[rel] = []Link{links, link}
		} else if links, ok := val.([]Link); ok {
			d.Links[rel] = append(links, link)
		} else {
			log.Fatal().Str("rel", rel).Interface("links", val).Msg("Links not of supported type")
		}
	} else {
		d.Links[rel] = link
	}

	return d
}

// Indicate the content type that HAL documents are in.
// This implements the WithContentType interface for the Response struct to use.
func (d Document) ContentType() string {
	return "application/hal+json"
}
