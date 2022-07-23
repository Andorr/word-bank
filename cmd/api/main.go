package main

import (
	"flag"
	"log"

	"github.com/Andorr/word-bank/api"
)

func main() {
	dbURI := flag.String("db", "", "database connection string")

	flag.Parse()

	if *dbURI == "" {
		log.Fatal("db connection string is required")
	}

	srv, err := api.NewServer(api.ServerOptions{
		Addr:  ":8080",
		DBURI: *dbURI,
	})
	if err != nil {
		log.Fatalf("failed to create server: %v", err)
	}

	if err := srv.Run(); err != nil {
		log.Fatalf("failed to run server: %v", err)
	}
}
