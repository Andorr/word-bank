use crate::{DB, Translation, Word, models::{PageResult, PaginationOptions, WordQueryOptions}};
use mongodb::{bson::{Bson, DateTime, doc, to_document}, options::FindOptions, sync::{Client, ClientSession, Collection, Database}};
use uuid::Uuid;

use super::models::{WordDBM, TranslationDBM};


const WORD_COL: &'static str = "words"; 

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

    fn start_transaction(&self) -> Result<ClientSession, ()> {
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
        return Ok(session)
    }
}

impl MongoDBClient {
    pub(crate) fn word_collection(&self) -> Collection<WordDBM> {
        self.db.collection::<WordDBM>(WORD_COL)
    }
}

impl DB for MongoDBClient {
    fn insert_word(&self, word: &mut Word) -> Result<Uuid, ()> {
        let wdbm: WordDBM = word.into();

        let collection = self.word_collection();
        match collection.insert_one(wdbm, None) {
            Ok(_) => {
                Ok(word.id)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
    
    fn insert_translation(&self, word_id: String, translation: &mut Translation) -> Result<Uuid, ()> {
        translation.id = Uuid::new_v4();
        let tdbm: TranslationDBM = (&*translation).into();

        let word_col = self.word_collection();
        let update_result = word_col.update_one(
            doc!{"_id": Bson::String(word_id)}, 
            doc!{"$push": doc!{
                "translations": to_document(&tdbm).unwrap(),
            }},None);

        match update_result {
            Ok(_) => {
                Ok(translation.id)
            }   
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
    
    fn query_words(&self, query_options: WordQueryOptions, pagination: PaginationOptions) -> Result<PageResult<Word>, ()> {
        let col = self.word_collection();
    
        let filter_options = FindOptions::builder()
            .sort(doc!{"created_at": -1})
            .skip(pagination.skip() as u64)
            .limit(pagination.limit as i64)
            .build();

        let result = col.find(query_options.clone().as_query_doc(), filter_options);
        if result.is_err() {
            println!("Find: {:?}", result.unwrap_err());
            return Err(())
        }
        let result_all = col.count_documents(query_options.as_query_doc(), None);
        if result.is_err() {
            println!("Count: {:?}", result.unwrap_err());
            return Err(())
        }
        let cursor = result.unwrap();
        let total_count = result_all.unwrap();

        let words: Vec<Word> = cursor.into_iter()
            .filter(|w| w.is_ok())
            .map(|w| w.unwrap())
            .map(|wdbms: WordDBM| wdbms.into())
            .collect();

        Ok(PageResult::<Word>{
            total: total_count as usize,
            page: pagination.page.clone(),
            count: words.len(),
            results: words,
        })
    }

    fn delete_word(&self, word_id: Uuid) -> Result<(), ()> {       
        // Delete both the word and its related translations
        let word_col = self.word_collection();
        let result_word_delete = word_col.delete_one(
            doc!{"_id": word_id.clone() },
            None
        );
        if result_word_delete.is_err() {
            return Err(());
        }
        
        Ok(())
    }
    
    fn update_word(&self, update_options: &crate::models::WordUpdateOptions) -> Result<(), ()> {
        let result = self.start_transaction();
        if result.is_err() {
            return Err(())
        }
        let mut session = result.unwrap();

        // Fetch word
        let word_col = self.word_collection();
        let word_result = word_col.find_one_with_session(
            doc!{ "_id": update_options.id.clone() }, 
            None,
            &mut session
        );
        if word_result.is_err() {
            return Err(())
        }
        let mut wdbm = match word_result.unwrap() {
            Some(w) => w,
            None => return Err(())
        };
        let updated_at = DateTime::now();
        wdbm.update(update_options, updated_at);
    

        let result = word_col.update_one_with_session(
            doc!{ "_id": wdbm._id.clone() }, 
            doc!{"$set": to_document(&wdbm).unwrap()},
            None, &mut session);
        if result.is_err() {
            let _ = session.abort_transaction();
            return Err(())
        }  

        if let Err(_) = session.commit_transaction() {
            return Err(())
        }
        
        Ok(())
    }
}