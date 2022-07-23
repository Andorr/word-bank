ifneq (,$(wildcard ./.env))
    include .env
    export
endif

run-api:
	go run ./cmd/api/main.go --db=${DB_URI}