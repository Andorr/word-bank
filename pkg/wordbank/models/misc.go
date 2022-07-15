package models

type PageResult[T any] struct {
	Total   int `json:"total"`
	Page    int `json:"page"`
	Count   int `json:"count"`
	Results []T `json:"results"`
}
