package main

import (
	"fmt"
	redis "github.com/docker/go-redis-server"
	config "tlog/config"
	"tlog/server"
	"tlog/backends"
)


func main() {

	defer func() {
		if msg := recover(); msg != nil {
			fmt.Printf("Panic: %v\n", msg)
		}
	}()



	// load config file
	conf, err := config.Load()

	if err != nil{
		panic(err)
	}


	var backend  interface{};

	// Get backend
	if conf.Backend == "file" {
		backend = &backends.File{}
	}else{
		backend = &backends.Zerostore{}
	}


	// T-log Object
	tlog := server.TLog{
		Config: conf,
		Namespaces: make(map[string]*server.Namespace),
		Backend: backend,
	}

	// Handler
	handler := &server.Handler{TLog: tlog}


	redisConf := redis.DefaultConfig()
	redisConf.Host(conf.Redis.Host)
	redisConf.Port(conf.Redis.Port)
	redisConf.Handler(handler)


	srv, err := redis.NewServer(redisConf)

	if err != nil {
		panic(err)
	}

	if err := srv.ListenAndServe(); err != nil {
		panic(err)
	}
}
