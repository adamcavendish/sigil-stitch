package config

// Config holds application configuration.
type Config struct {
	Name string `json:"name"`
	Port int `json:"port" yaml:"port"`
	Debug bool `json:"debug,omitempty"`
}
