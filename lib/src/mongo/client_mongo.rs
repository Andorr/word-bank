use crate::{DB, Translation, Word, models::{PageResult, PaginationOptions, WordQueryOptions}};
use mongodb::{bson::{self, Document, doc}, sync::{Client, ClientSession, Collection, Database}};

use super::models::{WordDBM, TranslationDBM};


const WORD_COL: &'static str = "words"; 
const TRANSLATION_COL: &'static str = "translations"; 

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
    
    pub(crate) fn translation_collection(&self) -> Collection<TranslationDBM> {
        self.db.collection::<TranslationDBM>(TRANSLATION_COL)
    }
}

impl DB for MongoDBClient {
    fn insert_word(&self, word: &mut Word) -> Result<String, ()> {
        let wdbm: WordDBM = word.into();

        let collection = self.word_collection();
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

        let collection = self.translation_collection();
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
        let result  = self.start_transaction();
        if result.is_err() {
            return Err(())
        }
        let mut session = result.unwrap();

        let word_col = self.word_collection();
        let translation_col = self.translation_collection();
        

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

    fn query_words(&self, query_options: WordQueryOptions, pagination: PaginationOptions) -> Result<PageResult<Word>, ()> {
        let col = self.word_collection();
        
        let pipeline: Vec<Document> = vec![
            query_options.clone().as_match_doc(),
            doc!{"$sort": doc!{"created_at": 1}},
            pagination.as_skip_doc(),
            pagination.as_limit_doc(),
            doc!{
                "$lookup": doc!{
                    "from": TRANSLATION_COL,
                    "localField": "_id",
                    "foreignField": "word_id",
                    "as": "translations"
                }
            }
        ];

        let result = col.aggregate(pipeline, None);
        if result.is_err() {
            println!("{:?}", result.unwrap_err());
            return Err(())
        }
        let result_all = col.count_documents(query_options.as_query_doc(), None);
        if result.is_err() {
            return Err(())
        }
        let cursor = result.unwrap();
        let total_count = result_all.unwrap();

        let words: Vec<Word> = cursor.into_iter()
            .filter(|w| w.is_ok())
            .map(|w| w.unwrap())
            .map(|w| bson::from_document(w).unwrap())
            .map(|wdbms: WordDBM| wdbms.into())
            .collect();

        Ok(PageResult::<Word>{
            total: total_count as usize,
            page: pagination.page.clone(),
            count: words.len(),
            results: words,
        })
    }

    fn delete_word(&self, word_id: String) -> Result<(), ()> {
        // Set up session, transactions and collections
        let result  = self.start_transaction();
        if result.is_err() {
            return Err(())
        }
        let mut session = result.unwrap();       

        // Delete both the word and its related translations
        let word_col = self.word_collection();
        let result_word_delete = word_col.delete_one_with_session(doc!{"_id": word_id.clone() }, None, &mut session);
        if result_word_delete.is_err() {
            let _ = session.abort_transaction();
            return Err(());
        }

        let translation_col = self.translation_collection();
        let result_translation_delete = translation_col.delete_one_with_session(doc!{"word_id": word_id }, None, &mut session);
        if result_translation_delete.is_err() {
            let _ = session.abort_transaction();
            return Err(())
        }

        if let Err(_) = session.commit_transaction() {
            return Err(())
        }

        Ok(())
    }
}