use std::str::FromStr;

use mongodb::bson::{DateTime, Document, doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

use crate::{Translation, Word, models::{WordFilterOptions, WordType}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDBM {
    pub _id: ObjectId,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: WordType, 
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationDBM {
    pub _id: ObjectId,
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
            id: wdbm._id.to_string(),
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
            _id: ObjectId::parse_str(word.id.as_str()).unwrap_or(ObjectId::new()),
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
        TranslationDBM { 
            _id: ObjectId::parse_str(translation.id.as_str()).unwrap_or(ObjectId::new()),
            word_id: ObjectId::from_str(translation.word_id.as_str()).unwrap_or(ObjectId::new()),
            value: translation.value.clone(),
            created_at: translation.created_at.into(),
            updated_at: translation.updated_at.into()
        }
    }
}

impl From<WordFilterOptions> for Document {
    fn from(options: WordFilterOptions) -> Self {
        let mut document = Document::new();
        
        if let Some(word) = options.word {
            document.insert("value", word);
        }
        if let Some(kind) = options.kind {
            document.insert("kind", kind.to_string());
        }
        if let Some(tags) = options.tags {
            document.insert("tags", tags);
        }

        document
    }
}