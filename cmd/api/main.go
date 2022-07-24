package main

import (
	"flag"
	"log"

	"github.com/Andorr/word-bank/api"
)

func main() {
	dbURI := flag.String("db", "", "database connection string")
	token := flag.String("token", "", "auth token secret")

	flag.Parse()

	if *dbURI == "" {
		log.Fatal("db connection string is required")
	}
	if *token == "" {
		log.Fatal("token is required")
	}

	srv, err := api.NewServer(api.ServerOptions{
		Addr:  ":8080",
		DBURI: *dbURI,
		Token: *token,
	})
	if err != nil {
		log.Fatalf("failed to create server: %v", err)
	}

	if err := srv.Run(); err != nil {
		log.Fatalf("failed to run server: %v", err)
	}
}
