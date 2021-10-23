use mongodb::bson::{DateTime, Document, doc};
use serde::{Serialize, Deserialize};

use mongodb::bson::serde_helpers::uuid_as_binary;
use crate::{PaginationOptions, Translation, Word, models::{WordQueryOptions, WordType, WordUpdateOptions}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: WordType, 
    pub tags: Vec<String>,
    pub translations: Vec<TranslationDBM>,
}

impl WordDBM {
    pub fn update(&mut self, update: &WordUpdateOptions, updated_at: DateTime) {
        if let Some(word) = update.word.clone() {
            self.value = word;
        }
        if let Some(kind) = update.kind {
            self.kind = kind;
        }
        if let Some(tags) = update.tags.clone() {
            self.tags = tags;
        }
        if let Some(translations) = update.translations.clone() {
            self.translations = translations.iter().map(|t| t.into()).collect();
        }
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,
    pub value: String,
}

impl From<WordDBM> for Word {
    fn from(wdbm: WordDBM) -> Self {
        Word { 
            id: wdbm._id,
            value: wdbm.value,
            kind: wdbm.kind,
            tags: wdbm.tags,
            translations: wdbm.translations.iter().map(|t| t.into()).collect(),
            created_at: wdbm.created_at.into(),
            updated_at: wdbm.updated_at.into(),

        }
    }
}

impl From<&mut Word> for WordDBM {
    fn from(word: &mut Word) -> Self {
        WordDBM {
            _id: word.id,
            value: word.value.clone(),
            kind: word.kind.clone(),
            tags: word.tags.clone(),
            translations: word.translations.iter().map(|t| t.into()).collect(),
            created_at: word.created_at.into(),
            updated_at: word.updated_at.into(),
        }
    }
}


impl From<&Translation> for TranslationDBM {
    fn from(translation: &Translation) -> Self {
        TranslationDBM { 
            _id: translation.id,
            value: translation.value.clone(),
        }
    }
}

impl From<& TranslationDBM> for Translation {
    fn from(translation: & TranslationDBM) -> Self {
        Translation { 
            id: translation._id,
            value: translation.value.clone(),
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

impl WordUpdateOptions {

    pub fn as_set_doc(&self, time: DateTime) -> Document {
        let mut doc = Document::new();
        
        if let Some(word) = self.word.clone() {
            doc.insert("value", word);
        }
        if let Some(kind) = self.kind {
            doc.insert("kind", kind.to_string());
        }
        if let Some(tags) = self.tags.clone() {
            doc.insert("tags", tags);
        }
        doc.insert("updated_at", time);
        doc
    }
}