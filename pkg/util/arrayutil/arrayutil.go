package arrayutil

func Map[T any, K any](arr []T, fn func(T) K) []K {
	o := make([]K, len(arr))
	for i, value := range arr {
		o[i] = fn(value)
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
