package rw

import "io"

type ReadWriter interface {
	io.Reader
	io.Writer
}
