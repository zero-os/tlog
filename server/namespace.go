package server

import "tlog/backends"

type Namespace struct {
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


func NewNameSpace() (*Namespace){

	return &Namespace{
		Queue: &Queue{},
		Data: &Data{},
		MetaData: &MetaData{},
	}

}