package service

import "net/http"

// Inject an HTTP Request into the service.
func (s Service) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.server.ServeHTTP(w, r)
}
