package server

import (
	"tlog/config"
	"tlog/backends"
)

type TLog struct{
	Config config.Config;
	Namespaces map[string]*Namespace
	Backend backends.Backend
}
