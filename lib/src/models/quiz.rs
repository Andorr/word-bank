use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Word;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuizMode {
    Normal,
    Endless,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizWordOption {
    pub folder_id: Option<Uuid>,
    pub count: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuizOptions {
    pub mode: QuizMode,
    pub words: QuizWordOption,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub words: Vec<Word>,
    pub options: QuizOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuizResult {
    pub timestamp: DateTime<Utc>,
    pub words: Vec<QuizWord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizWord {
    pub word_id: Uuid,
    pub num_success: usize,
    pub num_mistakes: usize,
}
