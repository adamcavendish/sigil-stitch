package models

type Endpoint struct {
	BaseConfig
	Path string
}

func URL(a *Endpoint) string {
	return fmt.Sprintf("%s/%s", a.Host, a.Path)
}
