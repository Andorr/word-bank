use chrono::{Utc};

pub use crate::models::{Word, Translation};
use crate::{DBOptions, mongo::MongoDBClient};

pub trait DB {
    fn insert_word(&self, word: &mut Word) -> Result<String, ()>;
    fn insert_translation(&self, translation: &mut Translation) -> Result<String, ()>;
    fn insert_word_with_translations(&self, word: &mut Word, translations: &mut Vec<Translation>) -> Result<String, ()>;
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

    pub fn new_word(&self, word: &mut Word, translations: &mut Vec<Translation>) -> Result<String, ()> {
        let now = Utc::now();
        word.update_time(now);
        translations
            .iter_mut()
            .for_each(|t| t.update_time(now));

        self.db.insert_word_with_translations(word, translations)
    }

}