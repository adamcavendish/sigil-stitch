package server

import "encoding/json"

// Server is an HTTP server.
type Server struct {
	host string
	port int
}

func (s *Server) Start() error {
	return nil
}

func (s *Server) ToJSON() ([]byte, error) {
	return json.Marshal(s)
}
