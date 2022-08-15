ifneq (,$(wildcard ./.env))
    include .env
    export
endif

run-api:
	go run ./cmd/api/main.go --db=${WORDBANK_DB_URI} --token=${WORDBANK_AUTH_TOKEN}