use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationOptionalOptions {
    pub limit: Option<usize>,
    pub page: Option<usize>,
}
