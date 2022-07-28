package utils

import (
	"fmt"
	"strings"
)

type StringScan[T any] interface {
	ScanString(value string) (T, error)
}

// PgScanArray scans a Postgres array into a slice of any type. The type T must implement StringScan[T].
func PgScanArray[T StringScan[T]](dst *[]T, value interface{}) error {
	source, ok := value.([]byte)
	if !ok {
		return fmt.Errorf("unable to convert %T to bytes", value)
	}

	var data []T = make([]T, 0)
	tokens := strings.Split(string(source), "\",\"")
	for _, token := range tokens {
		// Remove unecessary tokens
		for _, unwanted := range []string{"\\\"", "\"", "{", "}", "(", ")"} {
			// TODO: This does not work if 'unwanted' is in the middle of an actual value.
			// For instance: ("My value with \" quote"). The middle \" should not be removed.
			token = strings.ReplaceAll(token, unwanted, "")
		}

		elem, err := (*new(T)).ScanString(token)
		if err != nil {
			return err
		}
		data = append(data, elem)
	}

	*dst = data
	return nil
}
