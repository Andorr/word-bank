use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatistics {
    pub word_count: u64,
    pub word_types: HashMap<String, u64>,
    pub quiz_count: u64,
}
