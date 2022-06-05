use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::datetime_serializer;

use super::word::Word;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuizMode {
    Normal,
    Endless,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizWordOption {
    pub folders: Option<Vec<Uuid>>,
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
#[serde(rename_all = "camelCase")]
pub struct QuizWord {
    pub word_id: Uuid,
    pub num_success: usize,
    pub num_mistakes: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizResult {
    pub id: Uuid,

    pub questions: Vec<QuizQuestionResult>,

    #[serde(with = "datetime_serializer")]
    pub created_at: DateTime<Utc>,
}

impl QuizResult {
    pub fn new(questions: Vec<QuizQuestionResult>) -> Self {
        QuizResult {
            id: uuid::Uuid::new_v4(),
            questions: questions,
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestionResult {
    pub word_id: Uuid,
    pub num_corrects: u64,
    pub num_incorrects: u64,
}
