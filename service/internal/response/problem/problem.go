package problem

// Representation of a Problem details.
type Problem struct {
	Type     string `json:"type,omitempty"`
	Title    string `json:"title,omitempty"`
	Status   int    `json:"status"`
	Detail   string `json:"detail,omitempty"`
	Instance string `json:"instance,omitempty"`
}

// Indicate the content type that Problem documents are in.
// This implements the WithContentType interface for the Response struct to use.
func (p Problem) ContentType() string {
	return "application/problem+json"
}

// Indicate the status code for the problem.
// This implements the WithStatusCode interface for the Response struct to use.
func (p Problem) StatusCode() int {
	return p.Status
}
