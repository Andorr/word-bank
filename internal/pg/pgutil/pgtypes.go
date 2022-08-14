package pgutil

import (
	"github.com/Andorr/word-bank/internal/arrayutil"
	"github.com/google/uuid"
	"github.com/lib/pq"
)

func UUIDArrayToString(arr []uuid.UUID) pq.StringArray {
	return pq.StringArray(arrayutil.Map(arr, func(id uuid.UUID) string { return id.String() }))
}

func StringArrayToUUID(arr pq.StringArray) []uuid.UUID {
	return arrayutil.Map(arr, func(id string) uuid.UUID { return uuid.MustParse(id) })
}

func InterfaceSlice[T any](arr []T) []interface{} {
	return arrayutil.Map(arr, func(id T) interface{} { return id })
}
