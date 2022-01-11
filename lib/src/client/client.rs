use chrono::Utc;
use uuid::Uuid;

pub use crate::models::{Folder, Translation, Word};
use crate::{
    models::{
        quiz::Quiz, quiz::QuizOptions, FolderContent, FolderQueryOptions, FolderUpdateOptions,
        PageResult, PaginationOptions, WordQueryOptions, WordUpdateOptions,
    },
    mongo::{DBOptions, MongoContext, MongoDBClient},
    DB,
};

const PAGINATION_LIMIT: usize = 10000;

pub type Context = MongoContext;
// pub type Context = Box<dyn WordBankContext>;

#[derive(Clone)]
pub struct WordBankClient {
    db: MongoDBClient,
}

impl WordBankClient {
    pub fn from_mongo(options: DBOptions) -> Result<WordBankClient, ()> {
        let client = MongoDBClient::from_options(&options)?;
        let wbclient = WordBankClient { db: client };
        Ok(wbclient)
    }

    pub fn new_context(&self) -> Result<Context, ()> {
        self.db.new_context()
    }

    pub fn insert_word(&self, ctx: &mut Context, word: &mut Word) -> Result<Uuid, ()> {
        let now = Utc::now();
        word.update_time(now);

        if word.translations.len() == 0 {
            return Err(());
        }

        self.db.insert_word(ctx, word)
    }

    pub fn query_words(
        &self,
        filter: WordQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Word>, ()> {
        self.db.query_words(filter, pagination)
    }

    pub fn delete_word(&self, ctx: &mut Context, word_id: Uuid) -> Result<(), ()> {
        match self.db.delete_word(ctx, word_id) {
            Ok(_) => (),
            Err(_) => return Err(()),
        };

        let folder_result = match self.db.query_folders(
            ctx,
            FolderQueryOptions {
                query: None,
                words: Some(vec![word_id.clone()]),
                parent: None,
            },
            PaginationOptions::new(PAGINATION_LIMIT, 1),
        ) {
            Ok(folder_result) => folder_result,
            Err(_) => return Err(()),
        };

        if folder_result.results.len() == 0 {
            return Ok(());
        }

        let results: Result<(), ()> = folder_result
            .results
            .into_iter()
            .map(|f| {
                self.db.update_folder(
                    ctx,
                    &FolderUpdateOptions {
                        id: f.id.clone(),
                        remove: Some(vec![word_id]),
                        name: None,
                        parent: None,
                        add: None,
                    },
                )
            })
            .collect();

        results
    }

    pub fn update_word(&self, update_options: WordUpdateOptions) -> Result<(), ()> {
        self.db.update_word(&update_options)
    }

    pub fn insert_folder(&self, ctx: &mut Context, folder: &mut Folder) -> Result<Uuid, ()> {
        let now = Utc::now();
        folder.update_time(now);

        self.db.insert_folder(ctx, folder)
    }

    pub fn query_folders(
        &self,
        ctx: &mut Context,
        filter: FolderQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Folder>, ()> {
        self.db.query_folders(ctx, filter, pagination)
    }

    pub fn update_folder(
        &self,
        ctx: &mut Context,
        update_options: FolderUpdateOptions,
    ) -> Result<(), ()> {
        self.db.update_folder(ctx, &update_options)
    }

    pub fn delete_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<(), ()> {
        let folder = match self.db.get_folder(ctx, folder_id) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        // Check if folder is empty
        if folder.words.len() > 0 {
            return Err(());
        }
        let children = match self.db.query_folders(
            ctx,
            FolderQueryOptions {
                query: None,
                words: None,
                parent: Some(folder.id),
            },
            PaginationOptions::new(PAGINATION_LIMIT, 1),
        ) {
            Ok(result) => result,
            Err(_) => return Err(()),
        };
        if children.results.len() > 0 {
            return Err(());
        }

        self.db.delete_folder(ctx, folder.id)
    }

    pub fn get_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<Folder, ()> {
        self.db.get_folder(ctx, folder_id)
    }

    pub fn get_folder_content(
        &self,
        ctx: &mut Context,
        folder: &Folder,
    ) -> Result<FolderContent, ()> {
        let folders = match self.db.query_folders(
            ctx,
            FolderQueryOptions {
                query: None,
                words: None,
                parent: Some(folder.id.clone()),
            },
            PaginationOptions::new(PAGINATION_LIMIT, 1),
        ) {
            Ok(pr) => pr,
            Err(_) => return Err(()),
        };

        let words = match self.db.get_words(folder.words.clone()) {
            Ok(words) => words,
            Err(_) => return Err(()),
        };

        Ok(FolderContent {
            words: words,
            folders: folders.results,
        })
    }

    pub fn initialize_quiz(&self, ctx: &mut Context, options: QuizOptions) -> Result<Quiz, ()> {
        let words = match options.words.folder_id {
            Some(f_id) => {
                let folder = match self.get_folder(ctx, f_id) {
                    Ok(f) => f,
                    Err(_) => return Err(()),
                };

                let content = match self.get_folder_content(ctx, &folder) {
                    Ok(f) => f,
                    Err(_) => return Err(()),
                };
                content.words
            }
            None => {
                if let Ok(w) = self.db.random_words(ctx, options.words.count.unwrap_or(16)) {
                    w
                } else {
                    return Err(());
                }
            }
        };

        Ok(Quiz {
            id: Uuid::new_v4(),
            words,
            options,
        })
    }
}
