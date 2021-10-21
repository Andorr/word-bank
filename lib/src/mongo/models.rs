use std::str::FromStr;

use mongodb::bson::{DateTime, Document, doc, oid::ObjectId};
use serde::{Serialize, Deserialize};

use crate::{PaginationOptions, Translation, Word, models::{WordQueryOptions, WordType}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDBM {
    pub _id: ObjectId,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: WordType, 
    pub tags: Vec<String>,

    #[serde(default, skip_serializing)]
    pub translations: Option<Vec<TranslationDBM>>,
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
            translations: wdbm.translations.unwrap_or(Vec::new()).iter().map(|t| t.into()).collect(),
            created_at: wdbm.created_at.into(),
            updated_at: wdbm.updated_at.into(),

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
            updated_at: word.updated_at.into(),

            translations: None,
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

impl From<& TranslationDBM> for Translation {
    fn from(translation: & TranslationDBM) -> Self {
        Translation { 
            id: translation._id.to_string(),
            word_id: translation.word_id.to_string(),
            value: translation.value.clone(),
            created_at: translation.created_at.into(),
            updated_at: translation.updated_at.into()
        }
    }
}

impl WordQueryOptions {
    pub fn as_query_doc(self) -> Document {
        let mut document = Document::new();
        
        if let Some(word) = self.word {
            document.insert("value", word);
        }
        if let Some(kind) = self.kind {
            document.insert("kind", kind.to_string());
        }
        if let Some(tags) = self.tags {
            document.insert("tags", tags);
        }

        document
    }

    pub fn as_match_doc(self) -> Document {
        doc!{
            "$match": self.as_query_doc(),
        }
    }
}

impl From<WordQueryOptions> for Document {
    fn from(options: WordQueryOptions) -> Self {
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

impl PaginationOptions {

    pub fn as_limit_doc(self) -> Document {
        doc! {
            "$limit": self.limit.clone() as u32,
        }
    }
    pub fn as_skip_doc(self) -> Document {
        doc! {
            "$skip": self.skip().clone() as u32,
        }
    }
}