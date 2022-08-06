##
## Build
##
FROM golang:1.18-buster AS build

WORKDIR /app

COPY go.mod ./
COPY go.sum ./
RUN go mod download

COPY internal/ ./internal
COPY pkg/ ./pkg
COPY api/ ./api
COPY cmd/api/ ./cmd/api

RUN go build -o /bin/api ./cmd/api/main.go

##
## Deploy
##
FROM gcr.io/distroless/base-debian10

WORKDIR /

COPY --from=build /bin/api /api

EXPOSE 8080

# USER nonroot:nonroot

ENTRYPOINT ["/api"]