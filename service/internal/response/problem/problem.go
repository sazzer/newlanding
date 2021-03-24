package problem

// Representation of an HTTP Problem Details.
type Problem struct {
	Type     string `json:"type,omitempty"`
	Title    string `json:"title,omitempty"`
	Status   int    `json:"status,omitempty"`
	Detail   string `json:"detail,omitempty"`
	Instance string `json:"instance,omitempty"`
}

func (p Problem) StatusCode() int {
	return p.Status
}

func (p Problem) ContentType() string {
	return "application/problem+json"
}
