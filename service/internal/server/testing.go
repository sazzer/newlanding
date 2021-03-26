package server

import "net/http"

// Inject an HTTP Request into the HTTP Server.
func (s Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.server.ServeHTTP(w, r)
}
