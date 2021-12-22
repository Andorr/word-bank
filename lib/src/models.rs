use std::{fmt, str::FromStr};
use uuid::{self, Uuid};

use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};
use crate::utils::datetime_serializer;

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
    DETERMINER,
    OTHER,
}


impl fmt::Display for WordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for WordType {

    type Err = ();

    fn from_str(input: &str) -> Result<WordType, Self::Err> {
        match input.to_uppercase().as_str() {
            "NONE"  => Ok(WordType::NONE),
            "NOUN"  => Ok(WordType::NOUN),
            "PRONOUN" => Ok(WordType::PRONOUN),
            "VERB" => Ok(WordType::VERB),
            "ADJECTIVE" => Ok(WordType::ADJECTIVE),
            "ADVERB" => Ok(WordType::ADVERB),
            "PREPOSITION" => Ok(WordType::PREPOSITION),
            "CONJUNCTION" => Ok(WordType::CONJUNCTION),
            "INTERJECTION" => Ok(WordType::INTERJECTION),
            "DETERMINER" => Ok(WordType::DETERMINER),
            "OTHER" => Ok(WordType::OTHER),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub id: Uuid,
    pub value: String,
    pub kind: WordType, 
    pub tags: Vec<String>,
    
    pub translations: Vec<Translation>,

    #[serde(with = "datetime_serializer")]
    pub created_at: DateTime<Utc>,

    #[serde(with = "datetime_serializer")]
    pub updated_at: DateTime<Utc>,
}

impl Word {
    pub fn from_value(value: &str) -> Word {
        Word {
            id: Uuid::new_v4(),
            value: value.trim().to_string(),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordQueryOptions {
    pub query: Option<String>,
    pub word: Option<String>,
    pub kind: Option<WordType>,
    pub tags: Option<Vec<String>>
}

impl WordQueryOptions {
    pub fn empty() -> WordQueryOptions {
        WordQueryOptions { query: None, word: None, kind: None, tags: None }
    }
}

impl Default for WordQueryOptions {
    fn default() -> Self {
        WordQueryOptions::empty()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationOptions {
    pub limit: usize,
    pub page: usize,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        Self {
            limit: 25,
            page: 1,
        }
    }
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