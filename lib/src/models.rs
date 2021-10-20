use std::fmt;

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
    pub id: String,
    
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
            id: String::new(),
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


#[derive(Debug, Clone)]
pub struct Translation {
    pub id: String,
    pub word_id: String,
    pub value: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Translation {
    pub fn from_value(value: &str) -> Translation {
        Translation {
            id: String::new(),
            value: value.to_string(),
            word_id: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_time(&mut self, time: DateTime<Utc>) {
        self.created_at = time;
        self.updated_at = time;
    }    
}

pub struct WordFilterOptions {
    pub word: Option<String>,
    pub kind: Option<WordType>,
    pub tags: Option<Vec<String>>
}

impl WordFilterOptions {
    pub fn empty() -> WordFilterOptions {
        WordFilterOptions { word: None, kind: None, tags: None }
    }
}

pub struct PaginationOptions {
    pub limit: usize,
    pub page: usize,
}

impl PaginationOptions {

    pub fn new(limit: usize, page: usize) -> PaginationOptions {
        PaginationOptions {
            limit: limit,
            page: page,
        }
    }

    pub fn default() -> PaginationOptions {
        PaginationOptions { limit: 10, page: 1 }
    }

    pub fn skip(&self) -> usize {
        return self.limit * (self.page - 1);
    }
}

pub struct PageResult<T> {
    pub total: usize,
    pub page: usize,
    pub count: usize,
    pub results: Vec<T>,
}