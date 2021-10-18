mod models;

pub use self::models::{Word, Translation};

use mongodb::{bson::{Bson, DateTime, doc}, sync::{Client, Database}};

// Exports




pub trait DB {
    fn insert_word(self, word: &mut Word, translations: &Vec<Translation>) -> Result<String, ()>;
}

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
    fn insert_word(self, word: &mut Word, translations: &Vec<Translation>) -> Result<String, ()> {
        word.created_at = DateTime::now();
        word.updated_at = DateTime::now();

        let collection = self.db.collection::<Word>("words");
        match collection.insert_one(word, None) {
            Ok(result) => {
                let id = result.inserted_id;
                Ok(id.to_string())
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
}