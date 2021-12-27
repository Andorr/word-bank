use mongodb::bson::{doc, DateTime};
use uuid::Uuid;

use crate::{Folder, models::FolderUpdateOptions};

use super::{MongoDBClient, models::FolderDBM};

impl MongoDBClient {

    pub fn handle_get_folder(&self, folder_id: Uuid) -> Result<Folder, ()> {
        let mut session = match self.start_transaction() {
            Ok(session) => session,
            Err(_) => return Err(())
        };        
        
        let folder_collection = self.folder_collection();
        match self.fetch_entity(folder_id, &folder_collection, &mut session) {
            Ok(f) => {
                let _ = self.close_transaction(&mut session, false);
                Ok(f.into())
            },
            Err(_) => {
                let _ = self.close_transaction(&mut session, true);
                Err(())
            }    
        }
    }

    pub fn handle_insert_folder(&self, folder: &mut Folder) -> Result<Uuid, ()> {
        let fdbm: FolderDBM = folder.into();

        let collection = self.folder_collection();
        match collection.insert_one(fdbm, None) {
            Ok(_) => Ok(folder.id),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        } 
    }

    pub fn handle_delete_folder(&self, folder_id: Uuid) -> Result<(), ()> {
        // Delete both the word and its related translations
        let folder_col = self.folder_collection();
        let result_folder_delete = folder_col.delete_one(
            doc!{"_id": folder_id.clone() },
            None
        );
        if result_folder_delete.is_err() {
            return Err(());
        }
        
        Ok(())
    }

    pub fn handle_update_folder(&self, update_options: &FolderUpdateOptions) -> Result<(), ()> {
        let mut session = match self.start_transaction() {
            Ok(session) => session,
            Err(_) => return Err(())
        };

        let folder_col = self.folder_collection();
        let mut fdbm = match self.fetch_entity::<FolderDBM>(update_options.id.clone(), &folder_col, &mut session) {
            Ok(result) => result,
            Err(_) => return Err(())
        };

        let updated_at = DateTime::now();
        fdbm.update(update_options, updated_at);
    
        let result = self.update_entity(
            fdbm._id.clone(), 
            &folder_col, &fdbm, &mut session);
        let _ = self.close_transaction(&mut session, result.is_err());
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}