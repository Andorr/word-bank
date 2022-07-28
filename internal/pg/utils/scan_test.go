package utils

import (
	"fmt"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

type test struct {
	ID string
	A  int
	B  int
}

var _ StringScan[*test] = (*test)(nil)

func (a *test) ScanString(value string) (*test, error) {
	tokens := strings.Split(value, ",")
	if len(tokens) != 3 {
		return nil, fmt.Errorf("invalid artist format")
	}

	var err error
	b := test{}
	b.ID = tokens[0]
	b.A, _ = strconv.Atoi(tokens[1])
	b.B, _ = strconv.Atoi(tokens[2])
	return &b, err
}

func TestPgScanArraySingleElement(t *testing.T) {

	input := "{\"(6b077236-9ed0-43d4-b201-69cf3b206f9f,10,20)\"}"

	var tests []*test
	err := PgScanArray(&tests, []byte(input))
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}

	assert.Equal(t, 1, len(tests))
	assert.Equal(t, "6b077236-9ed0-43d4-b201-69cf3b206f9f", tests[0].ID)
	assert.Equal(t, 10, tests[0].A)
	assert.Equal(t, 20, tests[0].B)
}

func TestPgScanArrayMultipleElements(t *testing.T) {

	input := "{\"(6b077236-9ed0-43d4-b201-69cf3b206f9f,10,20)\",\"(a7c92157-1577-4cde-aec2-2295285bb9b6,30,20)\"}"

	var tests []*test
	err := PgScanArray(&tests, []byte(input))
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}

	if !assert.Equal(t, 2, len(tests)) {
		assert.FailNow(t, "unexpected number of elements")
	}

	assert.Equal(t, "6b077236-9ed0-43d4-b201-69cf3b206f9f", tests[0].ID)
	assert.Equal(t, 10, tests[0].A)
	assert.Equal(t, 20, tests[0].B)

	assert.Equal(t, "a7c92157-1577-4cde-aec2-2295285bb9b6", tests[1].ID)
	assert.Equal(t, 30, tests[1].A)
	assert.Equal(t, 20, tests[1].B)

}
