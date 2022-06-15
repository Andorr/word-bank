use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Error, Pool, PooledConnection},
};

use crate::DB;

use super::context::PgContext;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PgDBClient {
    pool: PgPool,
}

impl PgDBClient {
    pub fn new(database_url: &str) -> Result<PgDBClient, ()> {
        match PgDBClient::new_db_pool(database_url) {
            Ok(pool) => Ok(PgDBClient { pool }),
            Err(_) => Err(()),
        }
    }

    fn new_db_pool(database_url: &str) -> Result<PgPool, r2d2::PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager)
    }
}

impl DB for PgDBClient {
    type Context = PgContext;

    fn new_context(&self) -> Result<Self::Context, ()> {
        let conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(err) => {
                return Err(());
            }
        };
        let context = PgContext { conn: conn };
        Ok(context)
    }

    fn insert_word(
        &self,
        ctx: &mut Self::Context,
        word: &mut crate::Word,
    ) -> Result<uuid::Uuid, ()> {
        todo!()
    }

    fn insert_translation(
        &self,
        word_id: String,
        translation: &mut crate::Translation,
    ) -> Result<uuid::Uuid, ()> {
        todo!()
    }

    fn query_words(
        &self,
        query_options: crate::WordQueryOptions,
        pagination: crate::PaginationOptions,
    ) -> Result<crate::PageResult<crate::Word>, ()> {
        todo!()
    }

    fn delete_word(&self, ctx: &mut Self::Context, word_id: uuid::Uuid) -> Result<(), ()> {
        todo!()
    }

    fn update_word(&self, update_options: &crate::WordUpdateOptions) -> Result<(), ()> {
        todo!()
    }

    fn get_words(&self, ids: Vec<uuid::Uuid>) -> Result<Vec<crate::Word>, ()> {
        todo!()
    }

    fn insert_folder(
        &self,
        ctx: &mut Self::Context,
        folder: &mut crate::Folder,
    ) -> Result<uuid::Uuid, ()> {
        todo!()
    }

    fn delete_folder(&self, ctx: &mut Self::Context, folder_id: uuid::Uuid) -> Result<(), ()> {
        todo!()
    }

    fn update_folder(
        &self,
        ctx: &mut Self::Context,
        update_options: &crate::FolderUpdateOptions,
    ) -> Result<(), ()> {
        todo!()
    }

    fn query_folders(
        &self,
        ctx: &mut Self::Context,
        query_options: crate::FolderQueryOptions,
        pagination: crate::PaginationOptions,
    ) -> Result<crate::PageResult<crate::Folder>, ()> {
        todo!()
    }

    fn get_folder(
        &self,
        ctx: &mut Self::Context,
        folder_id: uuid::Uuid,
    ) -> Result<crate::Folder, ()> {
        todo!()
    }

    fn random_words(&self, ctx: &mut Self::Context, count: u32) -> Result<Vec<crate::Word>, ()> {
        todo!()
    }

    fn insert_quiz_result(
        &self,
        ctx: &mut Self::Context,
        results: &mut crate::quiz::QuizResult,
    ) -> Result<uuid::Uuid, ()> {
        todo!()
    }

    fn get_user_statistics(
        &self,
        ctx: &mut Self::Context,
    ) -> Result<crate::models::stats::UserStatistics, ()> {
        todo!()
    }
}
