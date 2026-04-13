package main

import (
	"encoding/json"
	"net/http"
)

func startServer() {
	srv := &http.Server{}
	data, _ := json.Marshal(srv)
	_ = data
}
