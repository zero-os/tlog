package server

import (
	"tlog/backends"
	"tlog/config"
)

type Namespace struct {
	Name string
	Queue *Queue
	Data *Data
	MetaData *MetaData
	Backend *backends.Backend
}


// Async
// Flush data into Backend, save metadata into Metadata
func (ns *Namespace) flusher(){

}

// Async
// Clean metadata older than specific time
func (ns *Namespace) cleaner(){

}

func (ns *Namespace) write(data[]byte) error{
	return ns.Data.write(data)

}

func (ns *Namespace) init(name string, globalConfig config.Config){
	var c = &config.NamespaceConfig{globalConfig}
	ns.Name = name
	ns.Queue = &Queue{}
	ns.Data = &Data{Config: c}
	ns.MetaData = &MetaData{Config: c}
}
