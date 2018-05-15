package server

import "tlog/config"

type MetaData struct {
	Config *config.NamespaceConfig

}

func NewMetaData() *MetaData{
	return &MetaData{

	}
}