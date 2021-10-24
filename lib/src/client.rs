use chrono::{Utc};
use uuid::Uuid;

pub use crate::models::{Word, Translation};
use crate::{DBOptions, models::{PageResult, PaginationOptions, WordQueryOptions, WordUpdateOptions}, mongo::MongoDBClient};

pub trait DB {
    fn insert_word(&self, word: &mut Word) -> Result<Uuid, ()>;
    fn insert_translation(&self, word_id: String, translation: &mut Translation) -> Result<Uuid, ()>;
    fn query_words(&self, query_options: WordQueryOptions, pagination: PaginationOptions) -> Result<PageResult<Word>, ()>;
    fn delete_word(&self, word_id: Uuid) -> Result<(), ()>;
    fn update_word(&self, update_options: &WordUpdateOptions) -> Result<(), ()>;
}

pub struct WordBankClient {
    db: Box<dyn DB + 'static>,
}

impl WordBankClient {

    pub fn from_mongo(options: DBOptions) -> Result<WordBankClient, ()> {
        let client = MongoDBClient::from_options(&options)?;
        let wbclient = WordBankClient {
            db: Box::new(client),
        };
        Ok(wbclient)
    }

    pub fn insert_word(&self, word: &mut Word) -> Result<Uuid, ()> {
        let now = Utc::now();
        word.update_time(now);

        if word.translations.len() == 0 {
            return Err(())
        }

        self.db.insert_word(word)
    }

    pub fn query_words(&self, filter: WordQueryOptions, pagination: PaginationOptions) -> Result<PageResult<Word>, ()> {
        self.db.query_words(filter, pagination)
    }

    pub fn delete_word(&self, word_id: Uuid) -> Result<(), ()> {
        self.db.delete_word(word_id)
    }

    pub fn update_word(&self, update_options: WordUpdateOptions) -> Result<(), ()> {
        self.db.update_word(&update_options)
    }
}