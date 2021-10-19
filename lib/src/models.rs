use chrono::{DateTime, Utc};

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WordType {
    None,
    Noun,
    Pronoun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
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
            kind: WordType::None,
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