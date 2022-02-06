use bson::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::quiz::{QuizQuestionResult, QuizResult};
use mongodb::bson::serde_helpers::uuid_as_binary;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizResultDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,

    pub questions: Vec<QuizQuestionResultDBM>,

    pub created_at: DateTime,
}

impl From<QuizResultDBM> for QuizResult {
    fn from(qr: QuizResultDBM) -> Self {
        QuizResult {
            id: qr._id,
            questions: qr.questions.into_iter().map(|q| q.into()).collect(),
            created_at: qr.created_at.into(),
        }
    }
}

impl From<&mut QuizResult> for QuizResultDBM {
    fn from(qr: &mut QuizResult) -> Self {
        QuizResultDBM {
            _id: qr.id,
            questions: qr.questions.clone().into_iter().map(|q| q.into()).collect(),
            created_at: qr.created_at.into(),
        }
    }
}

impl From<QuizQuestionResult> for QuizQuestionResultDBM {
    fn from(q: QuizQuestionResult) -> Self {
        QuizQuestionResultDBM {
            word_id: q.word_id,
            num_corrects: q.num_corrects,
            num_incorrects: q.num_incorrects,
        }
    }
}

impl From<QuizQuestionResultDBM> for QuizQuestionResult {
    fn from(q: QuizQuestionResultDBM) -> Self {
        QuizQuestionResult {
            word_id: q.word_id,
            num_corrects: q.num_corrects,
            num_incorrects: q.num_incorrects,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizQuestionResultDBM {
    #[serde(with = "uuid_as_binary")]
    pub word_id: Uuid,
    pub num_corrects: u64,
    pub num_incorrects: u64,
}
