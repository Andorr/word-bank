use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationOptions {
    pub limit: usize,
    pub page: usize,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        Self { limit: 25, page: 1 }
    }
}

impl PaginationOptions {
    pub fn new(limit: usize, page: usize) -> PaginationOptions {
        PaginationOptions {
            limit: limit,
            page: if page != 0 { page } else { 1 }, // Page == 0 is not allowed
        }
    }

    pub fn default() -> PaginationOptions {
        PaginationOptions { limit: 10, page: 1 }
    }

    pub fn skip(&self) -> usize {
        return self.limit * (self.page - 1);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub total: usize,
    pub page: usize,
    pub count: usize,
    pub results: Vec<T>,
}
