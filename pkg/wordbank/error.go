package wordbank

type ErrorCode string

type WordBankError struct {
	Status int
	Code   ErrorCode
	Err    error
}

var _ error = (*WordBankError)(nil)

func (e WordBankError) Error() string {
	return e.Err.Error()
}

func errServerError(err error) *WordBankError {
	if err == nil {
		return nil
	}
	return &WordBankError{Status: 500, Err: err}
}

func errBadRequest(code ErrorCode, err error) *WordBankError {
	if err == nil {
		return nil
	}
	return &WordBankError{Status: 400, Code: code, Err: err}
}

func errNotFound(err error) *WordBankError {
	if err == nil {
		return nil
	}
	return &WordBankError{Status: 404, Err: err}
}

func errServerErrorWithValue[T any](value T, err error) (T, *WordBankError) {
	if err == nil {
		return value, nil
	}
	return value, &WordBankError{Status: 500, Err: err}
}
