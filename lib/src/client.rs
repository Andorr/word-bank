use chrono::{Utc};
use uuid::Uuid;

pub use crate::models::{Word, Translation, Folder};
use crate::{models::{PageResult, PaginationOptions, WordQueryOptions, WordUpdateOptions, FolderUpdateOptions}, mongo::{MongoDBClient, DBOptions}};

pub trait DB {
    fn insert_word(&self, word: &mut Word) -> Result<Uuid, ()>;
    fn insert_translation(&self, word_id: String, translation: &mut Translation) -> Result<Uuid, ()>;
    fn query_words(&self, query_options: WordQueryOptions, pagination: PaginationOptions) -> Result<PageResult<Word>, ()>;
    fn delete_word(&self, word_id: Uuid) -> Result<(), ()>;
    fn update_word(&self, update_options: &WordUpdateOptions) -> Result<(), ()>;
    fn get_words(&self, ids: Vec<Uuid>) -> Result<Vec<Word>, ()>;

    fn insert_folder(&self, folder: &mut Folder) -> Result<Uuid, ()>;
    fn delete_folder(&self, folder_id: Uuid) -> Result<(), ()>;
    fn update_folder(&self, update_options: &FolderUpdateOptions) -> Result<(), ()>;
    fn get_folder(&self, folder_id: Uuid) -> Result<Folder, ()>;
}

#[derive(Clone)]
pub struct WordBankClient {
    // db: Box<dyn DB + 'static>,
    db: MongoDBClient,
}

impl WordBankClient {

    pub fn from_mongo(options: DBOptions) -> Result<WordBankClient, ()> {
        let client = MongoDBClient::from_options(&options)?;
        let wbclient = WordBankClient {
            db: client,
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

    pub fn insert_folder(&self, folder: &mut Folder) -> Result<Uuid, ()> {
        let now = Utc::now();
        folder.update_time(now);


        self.db.insert_folder(folder)
    }

    pub fn update_folder(&self, update_options: FolderUpdateOptions) -> Result<(), ()> {
        self.db.update_folder(&update_options)
    }

    pub fn delete_folder(&self, folder_id: Uuid) -> Result<(), ()> {
        let folder = match self.db.get_folder(folder_id) {
            Ok(f) => f,
            Err(_) => return Err(())
        };
        
        if folder.words.len() > 0 {
            return Err(())
        }


        self.db.delete_folder(folder.id)
    }
}