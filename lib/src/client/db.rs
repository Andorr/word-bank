use uuid::Uuid;

use crate::{
    models::{stats::UserStatistics, FolderQueryOptions},
    quiz::QuizResult,
    Folder, FolderUpdateOptions, PageResult, PaginationOptions, Translation, Word,
    WordQueryOptions, WordUpdateOptions,
};

use super::Context;

pub trait DB {
    fn new_context(&self) -> Result<Context, ()>;
    fn insert_word(&self, ctx: &mut Context, word: &mut Word) -> Result<Uuid, ()>;
    fn insert_translation(
        &self,
        word_id: String,
        translation: &mut Translation,
    ) -> Result<Uuid, ()>;
    fn query_words(
        &self,
        query_options: WordQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Word>, ()>;
    fn delete_word(&self, ctx: &mut Context, word_id: Uuid) -> Result<(), ()>;
    fn update_word(&self, update_options: &WordUpdateOptions) -> Result<(), ()>;
    fn get_words(&self, ids: Vec<Uuid>) -> Result<Vec<Word>, ()>;

    fn insert_folder(&self, ctx: &mut Context, folder: &mut Folder) -> Result<Uuid, ()>;
    fn delete_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<(), ()>;
    fn update_folder(
        &self,
        ctx: &mut Context,
        update_options: &FolderUpdateOptions,
    ) -> Result<(), ()>;
    fn query_folders(
        &self,
        ctx: &mut Context,
        query_options: FolderQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Folder>, ()>;
    fn get_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<Folder, ()>;

    // --- QUIZ RELATED ---
    fn random_words(&self, ctx: &mut Context, count: u32) -> Result<Vec<Word>, ()>;
    fn insert_quiz_result(&self, ctx: &mut Context, results: &mut QuizResult) -> Result<Uuid, ()>;

    // --- STATISTICS RELATED ---
    fn get_user_statistics(&self, ctx: &mut Context) -> Result<UserStatistics, ()>;
}
