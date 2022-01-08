use crate::{
    client::Context,
    models::{PageResult, PaginationOptions, WordQueryOptions},
    Folder, Translation, Word, WordUpdateOptions, DB,
};
use mongodb::{
    bson::{doc, to_document},
    error::Error,
    options::FindOptions,
    results::UpdateResult,
    sync::{Client, ClientSession, Collection, Database},
};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use super::{
    context::MongoContext,
    models::{FolderDBM, WordDBM},
};

const WORD_COL: &'static str = "words";
const FOLDER_COL: &'static str = "folders";

pub struct DBOptions {
    pub uri: String,
    pub database: String,
}

#[derive(Clone)]
pub struct MongoDBClient {
    client: Client,
    db: Database,
}

impl MongoDBClient {
    pub fn from_options(options: &DBOptions) -> Result<MongoDBClient, ()> {
        let client = Client::with_uri_str(options.uri.as_str()).unwrap();
        let database = client.database(options.database.as_str());

        let c = MongoDBClient {
            client: client,
            db: database,
        };

        Ok(c)
    }
}

impl MongoDBClient {
    pub(crate) fn word_collection(&self) -> Collection<WordDBM> {
        self.db.collection::<WordDBM>(WORD_COL)
    }

    pub(crate) fn folder_collection(&self) -> Collection<FolderDBM> {
        self.db.collection::<FolderDBM>(FOLDER_COL)
    }
}

impl DB for MongoDBClient {
    fn new_context(&self) -> Result<Context, ()> {
        let result = self.client.start_session(None);
        if result.is_err() {
            return Err(());
        }
        let mut session = result.unwrap();
        let result = session.start_transaction(None);
        if result.is_err() {
            return Err(());
        }
        Ok(MongoContext::new(session))
    }

    fn insert_word(&self, ctx: &mut Context, word: &mut Word) -> Result<Uuid, ()> {
        self.handle_insert_word(ctx, word)
    }

    fn insert_translation(
        &self,
        word_id: String,
        translation: &mut Translation,
    ) -> Result<Uuid, ()> {
        self.handle_insert_translation(word_id, translation)
    }

    fn query_words(
        &self,
        query_options: WordQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Word>, ()> {
        let col = self.word_collection();

        let filter_options = FindOptions::builder()
            .sort(doc! {"created_at": -1})
            .skip(pagination.skip() as u64)
            .limit(pagination.limit as i64)
            .build();

        let result = col.find(query_options.clone().as_query_doc(), filter_options);
        if result.is_err() {
            // println!("Find: {:?}", result.unwrap_err());
            return Err(());
        }
        let result_all = col.count_documents(query_options.as_query_doc(), None);
        if result.is_err() {
            // println!("Count: {:?}", result.unwrap_err());
            return Err(());
        }
        let cursor = result.unwrap();
        let total_count = result_all.unwrap();

        let words: Vec<Word> = cursor
            .into_iter()
            .filter(|w| w.is_ok())
            .map(|w| w.unwrap())
            .map(|wdbms: WordDBM| wdbms.into())
            .collect();

        Ok(PageResult::<Word> {
            total: total_count as usize,
            page: pagination.page.clone(),
            count: words.len(),
            results: words,
        })
    }

    fn delete_word(&self, ctx: &mut Context, word_id: Uuid) -> Result<(), ()> {
        self.handle_delete_word(ctx, word_id)
    }

    fn update_word(&self, update_options: &WordUpdateOptions) -> Result<(), ()> {
        self.handle_update_word(update_options)
    }

    fn get_words(&self, ids: Vec<Uuid>) -> Result<Vec<Word>, ()> {
        self.handle_get_words(ids)
    }

    fn random_words(&self, ctx: &mut Context, count: u32) -> Result<Vec<Word>, ()> {
        self.handle_get_random_words(ctx, count)
    }

    // ---- FOLDER IMPLEMENTATIONS ----

    fn insert_folder(&self, ctx: &mut Context, folder: &mut Folder) -> Result<Uuid, ()> {
        self.handle_insert_folder(ctx, folder)
    }

    fn delete_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<(), ()> {
        self.handle_delete_folder(ctx, folder_id)
    }

    fn update_folder(
        &self,
        ctx: &mut Context,
        update_options: &crate::models::FolderUpdateOptions,
    ) -> Result<(), ()> {
        self.handle_update_folder(ctx, update_options)
    }

    fn query_folders(
        &self,
        ctx: &mut Context,
        query_options: crate::models::FolderQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Folder>, ()> {
        self.handle_query_folders(ctx, query_options, pagination)
    }

    fn get_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<Folder, ()> {
        self.handle_get_folder(ctx, folder_id)
    }
}

impl MongoDBClient {
    pub fn fetch_entity<T>(
        &self,
        uuid: Uuid,
        collection: &Collection<T>,
        session: &mut ClientSession,
    ) -> Result<T, ()>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
    {
        let fetch_result = collection.find_one_with_session(doc! { "_id": uuid }, None, session);
        if fetch_result.is_err() {
            return Err(());
        }
        let fdbm = match fetch_result.unwrap() {
            Some(f) => f,
            None => return Err(()),
        };

        return Ok(fdbm);
    }

    pub fn update_entity<T>(
        &self,
        uuid: Uuid,
        collection: &Collection<T>,
        data: &T,
        session: &mut ClientSession,
    ) -> Result<UpdateResult, Error>
    where
        T: Serialize,
    {
        collection.update_one_with_session(
            doc! { "_id": uuid },
            doc! {"$set": to_document(data).unwrap()},
            None,
            session,
        )
    }

    pub fn start_transaction(&self) -> Result<ClientSession, ()> {
        // Set up session, transactions and collections
        let result = self.client.start_session(None);
        if result.is_err() {
            return Err(());
        }
        let mut session = result.unwrap();
        let result = session.start_transaction(None);
        if result.is_err() {
            return Err(());
        }
        return Ok(session);
    }

    pub fn close_transaction(&self, session: &mut ClientSession, abort: bool) -> Result<(), ()> {
        if abort {
            if let Err(_) = session.abort_transaction() {
                return Err(());
            }
        } else if let Err(_) = session.commit_transaction() {
            return Err(());
        }
        return Ok(());
    }
}
