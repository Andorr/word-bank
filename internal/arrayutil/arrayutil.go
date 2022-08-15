package arrayutil

func Map[T any, K any](arr []T, fn func(T) K) []K {
	o := make([]K, len(arr))
	for i, value := range arr {
		o[i] = fn(value)
	}
	return o
}

func Filter[T any](arr []T, fn func(T) bool) []T {
	o := make([]T, 0)
	for _, value := range arr {
		if fn(value) {
			o = append(o, value)
		}
	}
	return o
}

func Find[T any](arr []T, fn func(T) bool) *T {
	for _, value := range arr {
		if fn(value) {
			return &value
		}
	}
	return nil
}

func Remove[T comparable](arr []T, fn func(T) bool) []T {
	o := make([]T, 0)
	for _, value := range arr {
		if !fn(value) {
			o = append(o, value)
		}
	}
	return o
}

func Contains[T comparable](arr []T, value T) bool {
	for _, v := range arr {
		if v == value {
			return true
		}
	}
	return false
}

func Difference[T comparable](arr1 []T, arr2 []T) []T {
	o := make([]T, 0)
	for _, value := range arr1 {
		if !Contains(arr2, value) {
			o = append(o, value)
		}
	}
	return o
}

func ValueOrDefault[T comparable](value []T, defaultValue []T) []T {
	if value != nil {
		return value
	}
	return defaultValue
}

func Flatten[T any](arr [][]T) []T {
	o := make([]T, 0)
	for _, value := range arr {
		o = append(o, value...)
	}
	return o
}
