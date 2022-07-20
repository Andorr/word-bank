package models

type PageResult[T any] struct {
	Total   int `json:"total"`
	Page    int `json:"page"`
	Count   int `json:"count"`
	Results []T `json:"results"`
}

type PaginationOptions struct {
	Page  int `json:"page"`
	Limit int `json:"limit"`
}

func (p *PaginationOptions) NotEmpty() bool {
	return p.Page >= 0 && p.Limit > 0
}
