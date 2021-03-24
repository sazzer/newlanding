package service

type HTTPConfig struct {
	Port uint16
}

// Configuration needed to build the service.
type Config struct {
	HTTP HTTPConfig
}
