package server

import "tlog/config"


type MetaData struct {
	Config *config.NamespaceConfig
	MD map[int8]map[int8]map[int8]map[int8][]byte

}


func(md *MetaData) insert(){

}