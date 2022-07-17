package utils

import (
	"database/sql/driver"
	"fmt"
	"strings"
)

func SliceToPgValue(slice []interface{}) (driver.Value, error) {
	values := make([]string, 0)
	for _, val := range slice {
		if val != nil {
			switch val.(type) {
			case string:
				values = append(values, fmt.Sprintf("'%s'", val))
			default:
				valuer, ok := val.(driver.Valuer)
				if ok {
					value, err := valuer.Value()
					if err != nil {
						return nil, err
					}
					values = append(values, fmt.Sprintf("%v", value))
				} else {
					values = append(values, fmt.Sprintf("%v", val))
				}
			}
		}
	}
	return fmt.Sprintf("(%s)", strings.Join(values, ",")), nil
}
