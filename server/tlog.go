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

func (tl *TLog) GetOrCreateNameSpace(name string) *Namespace{
	ns := tl.Namespaces[name]
	if ns == nil{
		ns =  &Namespace{}
		ns.init(name, tl.Config)
		tl.Namespaces[name] = ns
	}
	return ns
}
