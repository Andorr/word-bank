use mongodb::bson::{DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: String, 
    pub tags: Vec<String>,

    #[serde(skip_serializing)]
    pub translations: Vec<Translation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub word_id: String,
    pub value: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Word {

    pub fn empty() ->  Word {
        Word {
            value: String::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            translations: Vec::new(),
            kind: String::new(), 
            tags: Vec::new()
        }
    }
}
