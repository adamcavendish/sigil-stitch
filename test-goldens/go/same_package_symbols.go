package main

import "net/http"

var _ http.Handler
var _ http.Server
_ = http.ListenAndServe
