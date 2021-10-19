mod models;

pub use self::models::{WordDBM, TranslationDBM};

use crate::{DB, Translation, Word};
use mongodb::{sync::{Client, Database}};


pub struct DBOptions<'a> {
    pub uri: &'a str,
    pub database: &'a str,
}

pub struct MongoDBClient {
    client: Client,
    db: Database, 
}

impl MongoDBClient {
    pub fn from_options(options: &DBOptions) -> Result<MongoDBClient, ()> {
        let client = Client::with_uri_str(options.uri).unwrap();
        let database = client.database(options.database);

        let c = MongoDBClient { 
            client: client,
            db: database,
        };

        Ok(c)   
    }
}

impl DB for MongoDBClient {
    fn insert_word(&self, word: &mut Word) -> Result<String, ()> {
        let wdbm: WordDBM = word.into();

        let collection = self.db.collection::<WordDBM>("words");
        match collection.insert_one(wdbm, None) {
            Ok(result) => {
                let id = result.inserted_id.as_object_id().unwrap().to_string();
                word.id = id;
                Ok(word.id.clone())
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
    
    fn insert_translation(&self, translation: &mut Translation) -> Result<String, ()> {
        let tdbm: TranslationDBM = translation.into();

        let collection = self.db.collection::<TranslationDBM>("translations");
        match collection.insert_one(tdbm, None) {
            Ok(result) => {
                let id = result.inserted_id.as_object_id().unwrap().to_string();
                translation.id = id;
                Ok(translation.id.clone())
            }   
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    fn insert_word_with_translations(&self, word: &mut Word, translations: &mut Vec<Translation>) -> Result<String, ()> {
        // Set up session, transactions and collections
        let result =  self.client.start_session(None);
        if result.is_err() {
            return Err(())
        }
        let mut session = result.unwrap();
        let result = session.start_transaction(None);
        if result.is_err() {
            return Err(())
        }
        
        let word_col = self.db.collection::<WordDBM>("words");
        let translation_col = self.db.collection::<TranslationDBM>("translations");
        

        // Insert word
        let wdbm: WordDBM = word.into(); 
        let word_result = match word_col.insert_one_with_session(wdbm, None, &mut session) {
            Ok(r) => r,
            Err(_) => {
                let _ = session.abort_transaction();
                return Err(())
            }
        };
        let word_id = word_result.inserted_id.as_object_id().unwrap();
        word.id = word_id.to_string();

        // Update translation foreign keys
        translations.iter_mut().for_each(|t| t.word_id = word.id.clone());

        // Insert translations
        let tdbms: Vec<TranslationDBM> = translations.iter_mut()
            .map::<TranslationDBM, _>(|t| t.into())
            .collect();

        let translation_result = match translation_col.insert_many_with_session(tdbms, None, &mut session) {
            Ok(r) => r,
            Err(_) => {
                let _  = session.abort_transaction();
                return Err(())
            }
        };

        // Update translation ids
        translation_result.inserted_ids
            .iter()
            .for_each(|(index, id)| {
                translations[*index].id = id.as_object_id().unwrap().to_string();
            });

        if let Err(_) = session.commit_transaction() {
            return Err(())
        }


        Ok(word.id.clone())
    }
}