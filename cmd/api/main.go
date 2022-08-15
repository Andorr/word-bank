package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/Andorr/word-bank/api"
)

func main() {
	dbURI := flag.String("db", "", "database connection string")
	token := flag.String("token", "", "auth token secret")

	flag.Parse()

	if *dbURI == "" {
		*dbURI = os.Getenv("WORDBANK_DB_URI")
		if *dbURI == "" {
			log.Fatal("db connection string is required")
		}
	}
	if *token == "" {
		*token = os.Getenv("WORDBANK_AUTH_TOKEN")
		if *token == "" {
			log.Fatal("token is required")
		}
	}

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	srv, err := api.NewServer(api.ServerOptions{
		Addr:  fmt.Sprintf(":%s", port),
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
