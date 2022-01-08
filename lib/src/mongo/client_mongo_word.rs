use mongodb::bson::{doc, to_document, Bson, DateTime};
use uuid::Uuid;

use crate::{client::Context, Translation, Word, WordUpdateOptions};

use super::{
    models::{TranslationDBM, WordDBM},
    MongoDBClient,
};

impl MongoDBClient {
    pub fn handle_insert_word(&self, ctx: &mut Context, word: &mut Word) -> Result<Uuid, ()> {
        let wdbm: WordDBM = word.into();

        let collection = self.word_collection();
        match collection.insert_one_with_session(wdbm, None, &mut ctx.session) {
            Ok(_) => Ok(word.id),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_insert_translation(
        &self,
        word_id: String,
        translation: &mut Translation,
    ) -> Result<Uuid, ()> {
        translation.id = Uuid::new_v4();
        let tdbm: TranslationDBM = (&*translation).into();

        let word_col = self.word_collection();
        let update_result = word_col.update_one(
            doc! {"_id": Bson::String(word_id)},
            doc! {"$push": doc!{
                "translations": to_document(&tdbm).unwrap(),
            }},
            None,
        );

        match update_result {
            Ok(_) => Ok(translation.id),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_delete_word(&self, ctx: &mut Context, word_id: Uuid) -> Result<(), ()> {
        // Delete both the word and its related translations
        let word_col = self.word_collection();
        let result_word_delete = word_col.delete_one_with_session(
            doc! {"_id": word_id.clone() },
            None,
            &mut ctx.session,
        );
        if result_word_delete.is_err() {
            return Err(());
        }

        Ok(())
    }

    pub fn handle_update_word(&self, update_options: &WordUpdateOptions) -> Result<(), ()> {
        let mut session = match self.start_transaction() {
            Ok(session) => session,
            Err(_) => return Err(()),
        };

        let word_col = self.word_collection();
        let mut wdbm = match self.fetch_entity::<WordDBM>(
            update_options.id.clone(),
            &word_col,
            &mut session,
        ) {
            Ok(result) => result,
            Err(_) => return Err(()),
        };

        let updated_at = DateTime::now();
        wdbm.update(update_options, updated_at);

        let result = self.update_entity(wdbm._id.clone(), &word_col, &wdbm, &mut session);
        let _ = self.close_transaction(&mut session, result.is_err());
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn handle_get_words(&self, ids: Vec<Uuid>) -> Result<Vec<Word>, ()> {
        let collection = self.word_collection();
        let result = collection.find(
            doc! {
                "_id": doc!{
                    "$in": ids,
                }
            },
            None,
        );

        match result {
            Ok(s) => Ok(s.map(|w| w.unwrap().into()).collect()),
            Err(_) => Err(()),
        }
    }

    pub fn handle_get_random_words(&self, ctx: &mut Context, count: u32) -> Result<Vec<Word>, ()> {
        let collection = self.word_collection();
        let mut result = match collection.aggregate_with_session(
            vec![doc! {
                "$sample": doc!{ "size": count as u32 },
            }],
            None,
            &mut ctx.session,
        ) {
            Ok(result) => result,
            Err(_) => return Err(()),
        };

        let words: Vec<Word> = result
            .iter(&mut ctx.session)
            .map(|f| bson::from_document::<WordDBM>(f.unwrap()).unwrap().into())
            .collect();

        Ok(words)
    }
}
