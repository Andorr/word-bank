use chrono::Utc;
use uuid::Uuid;

pub use crate::models::{Folder, Translation, Word};
use crate::{
    models::{
        quiz::Quiz, quiz::QuizOptions, stats::UserStatistics, FolderContent, FolderQueryOptions,
        FolderUpdateOptions, PageResult, PaginationOptions, WordQueryOptions, WordUpdateOptions,
    },
    mongo::{DBOptions, MongoDBClient},
    psql::PgDBClient,
    quiz::QuizResult,
    DB,
};

const PAGINATION_LIMIT: usize = 10000;

// pub type Context = PgContext;
// pub type Context = Box<dyn WordBankContext>;

#[derive(Clone)]
pub struct WordBankClient<T: DB> {
    db: T,
}

impl WordBankClient<MongoDBClient> {
    pub fn new(options: DBOptions) -> Result<WordBankClient<MongoDBClient>, ()> {
        let client = MongoDBClient::from_options(&options)?;
        let wbclient = WordBankClient { db: client };
        Ok(wbclient)
    }
}

impl WordBankClient<PgDBClient> {
    pub fn new(options: DBOptions) -> Result<WordBankClient<PgDBClient>, ()> {
        let client = PgDBClient::new(&options.uri)?;
        let wbclient = WordBankClient { db: client };
        Ok(wbclient)
    }
}

impl<T: DB> WordBankClient<T> {
    pub fn new_context(&self) -> Result<T::Context, ()> {
        self.db.new_context()
    }

    pub fn insert_word(&self, ctx: &mut T::Context, word: &mut Word) -> Result<Uuid, ()> {
        let now = Utc::now();
        word.update_time(now);

        if word.translations.len() == 0 {
            return Err(());
        }

        self.db.insert_word(ctx, word)
    }

    pub fn query_words(
        &self,
        ctx: &mut T::Context,
        filter: WordQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Word>, ()> {
        self.db.query_words(ctx, filter, pagination)
    }

    pub fn delete_word(&self, ctx: &mut T::Context, word_id: Uuid) -> Result<(), ()> {
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
                ids: None,
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

    pub fn update_word(
        &self,
        ctx: &mut T::Context,
        update_options: WordUpdateOptions,
    ) -> Result<Word, ()> {
        self.db.update_word(ctx, &update_options)
    }

    pub fn get_random_words(&self, ctx: &mut T::Context, count: u32) -> Result<Vec<Word>, ()> {
        self.db.random_words(ctx, count)
    }

    pub fn insert_folder(&self, ctx: &mut T::Context, folder: &mut Folder) -> Result<Uuid, ()> {
        let now = Utc::now();
        folder.update_time(now);

        self.db.insert_folder(ctx, folder)
    }

    pub fn query_folders(
        &self,
        ctx: &mut T::Context,
        filter: FolderQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Folder>, ()> {
        self.db.query_folders(ctx, filter, pagination)
    }

    pub fn update_folder(
        &self,
        ctx: &mut T::Context,
        update_options: FolderUpdateOptions,
    ) -> Result<(), ()> {
        self.db.update_folder(ctx, &update_options)
    }

    pub fn delete_folder(&self, ctx: &mut T::Context, folder_id: Uuid) -> Result<(), ()> {
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
                ids: None,
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

    pub fn get_folder(&self, ctx: &mut T::Context, folder_id: Uuid) -> Result<Folder, ()> {
        self.db.get_folder(ctx, folder_id)
    }

    pub fn get_folder_content(
        &self,
        ctx: &mut T::Context,
        folder: &Folder,
    ) -> Result<FolderContent, ()> {
        let folders = match self.db.query_folders(
            ctx,
            FolderQueryOptions {
                query: None,
                words: None,
                parent: Some(folder.id.clone()),
                ids: None,
            },
            PaginationOptions::new(PAGINATION_LIMIT, 1),
        ) {
            Ok(pr) => pr,
            Err(_) => return Err(()),
        };

        let words = match self.db.get_words(ctx, folder.words.clone()) {
            Ok(words) => words,
            Err(_) => return Err(()),
        };

        Ok(FolderContent {
            words: words,
            folders: folders.results,
        })
    }

    pub fn initialize_quiz(&self, ctx: &mut T::Context, options: &QuizOptions) -> Result<Quiz, ()> {
        let words = match &options.words.folders {
            Some(f_ids) => {
                let word_ids: Vec<Uuid> = match self.db.query_folders(
                    ctx,
                    FolderQueryOptions::empty().ids(f_ids.clone()),
                    PaginationOptions::new(10, 1),
                ) {
                    Ok(fs) => fs
                        .results
                        .iter()
                        .map(|f| f.words.clone())
                        .flatten()
                        .collect(),
                    Err(_) => return Err(()),
                };

                match self.db.get_words(ctx, word_ids) {
                    Ok(f) => f,
                    Err(_) => return Err(()),
                }
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
            options: options.clone(),
        })
    }

    pub fn insert_quiz_result(
        &self,
        ctx: &mut T::Context,
        result: &mut QuizResult,
    ) -> Result<Uuid, ()> {
        self.db.insert_quiz_result(ctx, result)
    }

    pub fn get_user_statistics(&self, ctx: &mut T::Context) -> Result<UserStatistics, ()> {
        self.db.get_user_statistics(ctx)
    }
}
