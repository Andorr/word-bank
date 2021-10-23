use std::fmt;
use uuid::{self, Uuid};

use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum WordType {
    NONE,
    NOUN,
    PRONOUN,
    VERB,
    ADJECTIVE,
    ADVERB,
    PREPOSITION,
    CONJUNCTION,
    INTERJECTION,
}

impl fmt::Display for WordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub id: Uuid,
    pub value: String,
    pub kind: WordType, 
    pub tags: Vec<String>,
    
    pub translations: Vec<Translation>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Word {
    pub fn from_value(value: &str) -> Word {
        Word {
            id: Uuid::new_v4(),
            value: value.to_string(),
            kind: WordType::NONE,
            tags: Vec::new(),
            translations: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_time(&mut self, time: DateTime<Utc>) {
        self.created_at = time;
        self.updated_at = time;
    }    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub id: Uuid,
    pub value: String,
}

impl Translation {
    pub fn from_value(value: &str) -> Translation {
        // Capitialize input
        let mut translation = String::from(value);
        if let Some(c) = (translation).get_mut(0..1) {
            c.make_ascii_uppercase();
        }

        Translation {
            id: Uuid::new_v4(),
            value: translation.to_string(),
        }
    } 
}

#[derive(Clone)]
pub struct WordQueryOptions {
    pub word: Option<String>,
    pub kind: Option<WordType>,
    pub tags: Option<Vec<String>>
}

impl WordQueryOptions {
    pub fn empty() -> WordQueryOptions {
        WordQueryOptions { word: None, kind: None, tags: None }
    }
}

#[derive(Clone, Copy)]
pub struct PaginationOptions {
    pub limit: usize,
    pub page: usize,
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

#[derive(Debug, Clone)]
pub struct WordUpdateOptions {
    pub id: Uuid,
    pub word: Option<String>,
    pub kind: Option<WordType>,
    pub tags: Option<Vec<String>>,
    pub translations: Option<Vec<Translation>>,
}