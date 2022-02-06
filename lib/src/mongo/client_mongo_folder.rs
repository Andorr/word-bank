use mongodb::{
    bson::{doc, DateTime},
    options::FindOptions,
};
use uuid::Uuid;

use crate::{
    client::Context,
    models::{FolderQueryOptions, FolderUpdateOptions},
    Folder, PageResult, PaginationOptions,
};

use super::{models::FolderDBM, MongoDBClient};

impl MongoDBClient {
    pub fn handle_query_folders(
        &self,
        ctx: &mut Context,
        query_options: FolderQueryOptions,
        pagination: PaginationOptions,
    ) -> Result<PageResult<Folder>, ()> {
        let col = self.folder_collection();

        let filter_options = FindOptions::builder()
            .sort(doc! {"created_at": -1 as i64})
            .skip(pagination.skip() as u64)
            .limit(pagination.limit as i64)
            .build();

        let result = col.find_with_session(
            query_options.clone().as_query_doc(),
            filter_options,
            &mut ctx.session,
        );
        if result.is_err() {
            return Err(());
        }
        let result_all =
            col.count_documents_with_session(query_options.as_query_doc(), None, &mut ctx.session);
        if result.is_err() {
            return Err(());
        }

        let mut cursor = result.unwrap();
        let total_count = result_all.unwrap();

        let folders: Vec<Folder> = cursor
            .iter(&mut ctx.session)
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap())
            .map(|fdbms: FolderDBM| fdbms.into())
            .collect();

        Ok(PageResult::<Folder> {
            total: total_count as usize,
            page: pagination.page.clone(),
            count: folders.len(),
            results: folders,
        })
    }

    pub fn handle_get_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<Folder, ()> {
        let folder_collection = self.folder_collection();
        match self.fetch_entity(folder_id, &folder_collection, &mut ctx.session) {
            Ok(f) => Ok(f.into()),
            Err(_) => Err(()),
        }
    }

    pub fn handle_insert_folder(&self, ctx: &mut Context, folder: &mut Folder) -> Result<Uuid, ()> {
        let fdbm: FolderDBM = folder.into();

        let collection = self.folder_collection();
        match collection.insert_one_with_session(fdbm, None, &mut ctx.session) {
            Ok(_) => Ok(folder.id),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_delete_folder(&self, ctx: &mut Context, folder_id: Uuid) -> Result<(), ()> {
        // Delete both the word and its related translations
        let folder_col = self.folder_collection();
        let result_folder_delete = folder_col.delete_one_with_session(
            doc! {"_id": folder_id.clone() },
            None,
            &mut ctx.session,
        );
        if result_folder_delete.is_err() {
            return Err(());
        }
        Ok(())
    }

    pub fn handle_update_folder(
        &self,
        ctx: &mut Context,
        update_options: &FolderUpdateOptions,
    ) -> Result<(), ()> {
        let folder_col = self.folder_collection();
        let mut fdbm = match self.fetch_entity::<FolderDBM>(
            update_options.id.clone(),
            &folder_col,
            &mut ctx.session,
        ) {
            Ok(result) => result,
            Err(_) => return Err(()),
        };

        let updated_at = DateTime::now();
        fdbm.update(update_options, updated_at);

        let result = self.update_entity(fdbm._id.clone(), &folder_col, &fdbm, &mut ctx.session);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
