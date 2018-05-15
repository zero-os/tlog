package config

import (
	"os"
	"encoding/json"
)

// redis config
type Redis struct{
	Host string `json: host`
	Port int `json: port`
}

// metadata config
type MetaData struct{
	MaxAgeInDays float32 `json:"max_age_in_days"`
}

// namespace config
type Namespace struct {

}

// Global config
type Config struct {
	MaxSizeB4Flush int16 `json: max_size_b4F_fush`;
	Redis Redis;
	Backend string`json: backend`
	MetaData MetaData
	Namespace Namespace
}


// Load global config from config.json file
func Load() (Config, error) {
	var config Config
	f, err := os.Open("config.json")

	defer f.Close()

	if err != nil{
		return config, err
	}

	json.NewDecoder(f).Decode(&config)

	return config, nil
}

// Namespace Local config
type NamespaceConfig struct {
	Global Config
}

func NewNameSpaceConfig(config Config) *NamespaceConfig{
	return &NamespaceConfig{
		Global: config,

	}
}