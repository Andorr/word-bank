use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Error, Pool, PooledConnection},
};

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use crate::{Word, DB};

use super::context::PgContext;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;
pub type PgPooledConnection = PooledConnection<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct PgDBClient {
    pool: PgPool,
}

impl PgDBClient {
    pub fn new(database_url: &str) -> Result<PgDBClient, ()> {
        let manager = PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);

        match r2d2::Pool::new(manager) {
            Ok(pool) => Ok(PgDBClient { pool }),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
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
        self.handle_insert_word(ctx, word)
    }

    fn query_words(
        &self,
        ctx: &mut Self::Context,
        query_options: crate::WordQueryOptions,
        pagination: crate::PaginationOptions,
    ) -> Result<crate::PageResult<crate::Word>, ()> {
        self.handle_query_words(ctx, &query_options, &pagination)
    }

    fn delete_word(&self, ctx: &mut Self::Context, word_id: uuid::Uuid) -> Result<(), ()> {
        self.handle_delete_word(ctx, word_id)
    }

    fn update_word(
        &self,
        ctx: &mut Self::Context,
        update_options: &crate::WordUpdateOptions,
    ) -> Result<Word, ()> {
        self.handle_update_word(ctx, update_options)
    }

    fn get_words(
        &self,
        ctx: &mut Self::Context,
        ids: Vec<uuid::Uuid>,
    ) -> Result<Vec<crate::Word>, ()> {
        self.handle_get_words(ctx, ids)
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
