use super::schema::words;
use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types,
    types::{FromSql, IsNull, ToSql},
    Queryable,
};
use diesel_derive_enum::DbEnum;

use std::convert::TryInto;

#[derive(SqlType)]
#[postgres(type_name = "word_kind")]
pub struct WordTypePGType;

#[derive(Debug, Clone, Copy, PartialEq, DbEnum)]
pub enum WordType {
    NONE,
    NOUN,
    PRONOUN,
    VERB,
    ADJECTIVE,
    ADVERB,
    PREPOSITION,
    CONJUNCTION,
    INTERJECTION,
    DETERMINER,
    OTHER,
}
#[derive(Queryable, Identifiable, Insertable, Debug)]
#[table_name = "words"]
pub struct Word {
    pub id: uuid::Uuid,
    pub word: String,
    pub kind: WordType,
    pub tags: Vec<String>,
    pub translations: Vec<Translation>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Folder {
    pub id: uuid::Uuid,
    pub name: String,
    pub words: Vec<uuid::Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, SqlType)]
#[postgres(type_name = "Translation")]
pub struct TranslationEntry;
#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "TranslationEntry"]
pub struct Translation {
    pub id: uuid::Uuid,
    pub value: String,
}

impl ToSql<TranslationEntry, Pg> for Translation {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        // Steps to write a custom type:
        // 1. Write the number of fields in the tuple
        ToSql::<sql_types::Integer, Pg>::to_sql(&2, out)?;

        // 2. For every field:
        //      1. Write the OID of the field - https://www.postgresql.org/message-id/20080708000535.GA10491@cuci.nl
        //      2. Write the length of the field
        //      3. Write the value of the field

        // For the field "id" - UUID
        ToSql::<sql_types::Oid, Pg>::to_sql(&2950, out)?;
        ToSql::<sql_types::Integer, Pg>::to_sql(&16, out)?;
        ToSql::<sql_types::Uuid, Pg>::to_sql(&self.id, out)?;

        // For the field "value" - str
        ToSql::<sql_types::Oid, Pg>::to_sql(&1043, out)?;
        ToSql::<sql_types::Integer, Pg>::to_sql(&(self.value.len() as i32), out)?;
        ToSql::<sql_types::VarChar, Pg>::to_sql(&self.value.as_str(), out)?;

        Ok(IsNull::No)

        // This can potentially be replaced with WriteTuple instead. Did not get it to work though :(
        // WriteTuple::<(uuid::Uuid, &str)>::write_tuple(&(self.id, self.value.as_str()), out)
    }
}

impl FromSql<TranslationEntry, Pg> for Translation {
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let values = bytes.unwrap().bytes_from_sql_tuple();
        let id = FromSql::<sql_types::Uuid, Pg>::from_sql(Some(values[0].as_slice()))?;
        let value: String =
            FromSql::<sql_types::VarChar, Pg>::from_sql(Some(values[1].as_slice()))?;
        Ok(Translation { id, value })
    }
}

trait FromSQLTuple {
    fn bytes_from_sql_tuple(&self) -> Vec<Vec<u8>>;
}

impl FromSQLTuple for [u8] {
    fn bytes_from_sql_tuple(&self) -> Vec<Vec<u8>> {
        let mut bytes: Vec<u8> = self.clone().to_vec();

        let mut values = Vec::new();
        let num_fields = u32::from_be_bytes(bytes.drain(0..4).as_slice().try_into().unwrap());
        for _ in 0..num_fields {
            let _ = u32::from_be_bytes(bytes.drain(0..4).as_slice().try_into().unwrap());
            let length = u32::from_be_bytes(bytes.drain(0..4).as_slice().try_into().unwrap());
            let value = bytes.drain(..(length as usize));
            values.push(value.collect());
        }

        values
    }
}
