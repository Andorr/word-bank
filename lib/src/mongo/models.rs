use std::str::FromStr;

use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Serialize, Deserialize};

use crate::{Translation, Word, models::WordType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDBM {
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: WordType, 
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationDBM {
    pub word_id: ObjectId,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}


impl WordDBM {

}

impl From<WordDBM> for Word {
    fn from(wdbm: WordDBM) -> Self {
        Word { 
            id: String::new(),
            value: wdbm.value,
            kind: wdbm.kind,
            tags: wdbm.tags,
            translations: Vec::new(),
            created_at: wdbm.created_at.into(),
            updated_at: wdbm.updated_at.into()
        }
    }
}

impl From<&mut Word> for WordDBM {
    fn from(word: &mut Word) -> Self {
        WordDBM { 
            value: word.value.clone(),
            kind: word.kind.clone(),
            tags: word.tags.clone(),
            created_at: word.created_at.into(),
            updated_at: word.updated_at.into()
        }
    }
}


impl From<&mut Translation> for TranslationDBM {
    fn from(translation: &mut Translation) -> Self {
        let id = ObjectId::from_str(translation.word_id.as_str()).unwrap_or(ObjectId::new());

        TranslationDBM { 
            word_id: id,
            value: translation.value.clone(),
            created_at: translation.created_at.into(),
            updated_at: translation.updated_at.into()
        }
    }
}