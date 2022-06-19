use super::models::{PgTranslation, PgTranslationEntry, PgWord, PgWordType, PgWordTypeMapping};
use super::schema::words;
use super::{PgContext, PgDBClient};

use chrono::{NaiveDateTime, Utc};
use diesel::dsl::Eq;
use diesel::expression::bound::Bound;
use diesel::prelude::*;
use diesel::query_builder::AsChangeset;
use diesel::sql_types::{Array, Text};
use diesel::types::ToSql;

impl PgDBClient {
    pub fn handle_insert_word(
        &self,
        ctx: &mut PgContext,
        new_word: &mut crate::Word,
    ) -> Result<uuid::Uuid, ()> {
        let wdbm: PgWord = new_word.into();

        match diesel::insert_into(words::table)
            .values(&wdbm)
            .get_result::<PgWord>(&ctx.conn)
        {
            Ok(_) => {
                new_word.id = wdbm.id;
                Ok(new_word.id)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }

    pub fn handle_delete_word(&self, ctx: &mut PgContext, word_id: uuid::Uuid) -> Result<(), ()> {
        // Delete both the word and its related translations
        diesel::delete(words::table)
            .filter(words::id.eq(word_id))
            .execute(&ctx.conn)
            .map(|_| ())
            .map_err(|err| {
                println!("{:?}", err);
                ()
            })
    }

    pub fn handle_update_word(
        &self,
        ctx: &mut PgContext,
        update_options: &crate::WordUpdateOptions,
    ) -> Result<PgWord, ()> {
        match diesel::update(words::table.find(update_options.id))
            .set(update_options)
            .get_result::<PgWord>(&ctx.conn)
        {
            Ok(word) => Ok(word),
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
}

impl AsChangeset for &crate::WordUpdateOptions {
    type Target = words::table;
    type Changeset = (
        Option<Eq<words::word, Bound<Text, String>>>,
        Option<Eq<words::kind, Bound<PgWordTypeMapping, PgWordType>>>,
        Option<Eq<words::tags, Vec<String>>>,
        Option<Eq<words::translations, Bound<Array<PgTranslationEntry>, Vec<PgTranslation>>>>,
        Option<Eq<words::updated_at, NaiveDateTime>>,
    );

    fn as_changeset(self) -> Self::Changeset {
        (
            self.word.clone().map(|f| words::word.eq(f)),
            self.kind.map(|f| words::kind.eq::<PgWordType>(f.into())),
            self.tags.clone().map(|f| words::tags.eq(f)),
            self.translations.clone().map(|t| {
                words::translations.eq::<Vec<PgTranslation>>(t.iter().map(|t| t.into()).collect())
            }),
            Some(words::updated_at.eq(Utc::now().naive_utc())),
        )
    }
}
