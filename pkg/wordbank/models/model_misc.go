package models

type PageResult[T any] struct {
	Total   int `json:"total"`
	Page    int `json:"page"`
	Count   int `json:"count"`
	Results []T `json:"results"`
}

type PaginationOptions struct {
	Page    int    `json:"page"`
	Limit   int    `json:"limit"`
	OrderBy string `json:"orderBy"`
}

type OrderByDirection string

const OrderByDirASC = "ASC"
const OrderByDirDESC = "DESC"

func (p *PaginationOptions) NotEmpty() bool {
	return p.Page > 0 && p.Limit > 0
}

func (p *PaginationOptions) OrderByField() string {
	if len(p.OrderBy) == 0 {
		return ""
	}
	if p.OrderBy[0] == '-' {
		return p.OrderBy[1:]
	}
	return p.OrderBy
}

func (p *PaginationOptions) OrderByDirection() OrderByDirection {
	if len(p.OrderBy) == 0 {
		return ""
	}
	if p.OrderBy[0] == '-' {
		return OrderByDirDESC
	}
	return OrderByDirASC
}
