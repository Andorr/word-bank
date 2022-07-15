use crate::{
    psql::sqlbuilder::{Insertable, Value},
    Translation, Word, WordType,
};

use chrono::{DateTime, Utc};
use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types,
    // types::{FromSql, IsNull, ToSql},
    Queryable,
};
use diesel_derive_enum::DbEnum;
use postgres_types::{FromSql, ToSql};
use r2d2_postgres::postgres::Row;

use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSql, ToSql)]
// #[PgType = "word_kind"]
// #[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum PgWordType {
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

impl std::fmt::Display for PgWordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let w: WordType = self.into();
        std::fmt::Debug::fmt(&w, f)
    }
}

impl From<&PgWordType> for WordType {
    fn from(pg_word_type: &PgWordType) -> Self {
        match pg_word_type {
            PgWordType::NONE => WordType::NONE,
            PgWordType::NOUN => WordType::NOUN,
            PgWordType::PRONOUN => WordType::PRONOUN,
            PgWordType::VERB => WordType::VERB,
            PgWordType::ADJECTIVE => WordType::ADJECTIVE,
            PgWordType::ADVERB => WordType::ADVERB,
            PgWordType::PREPOSITION => WordType::PREPOSITION,
            PgWordType::CONJUNCTION => WordType::CONJUNCTION,
            PgWordType::INTERJECTION => WordType::INTERJECTION,
            PgWordType::DETERMINER => WordType::DETERMINER,
            PgWordType::OTHER => WordType::OTHER,
        }
    }
}

impl From<WordType> for PgWordType {
    fn from(word_type: WordType) -> Self {
        match word_type {
            WordType::NONE => PgWordType::NONE,
            WordType::NOUN => PgWordType::NOUN,
            WordType::PRONOUN => PgWordType::PRONOUN,
            WordType::VERB => PgWordType::VERB,
            WordType::ADJECTIVE => PgWordType::ADJECTIVE,
            WordType::ADVERB => PgWordType::ADVERB,
            WordType::PREPOSITION => PgWordType::PREPOSITION,
            WordType::CONJUNCTION => PgWordType::CONJUNCTION,
            WordType::INTERJECTION => PgWordType::INTERJECTION,
            WordType::DETERMINER => PgWordType::DETERMINER,
            WordType::OTHER => PgWordType::OTHER,
        }
    }
}

// impl<'a> From<PgWordType> for &'a Value {
//     fn from(pg_word_type: PgWordType) -> Self {
//         &pg_word_type.to_string()
//     }
// }

#[derive(Debug)]
pub struct PgWord {
    pub id: uuid::Uuid,
    pub word: String,
    pub kind: PgWordType,
    pub tags: Vec<String>,
    pub translations: Vec<PgTranslation>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<&mut Word> for PgWord {
    fn from(word: &mut Word) -> Self {
        PgWord {
            id: word.id,
            word: word.value.clone(),
            kind: word.kind.clone().into(),
            tags: word.tags.clone(),
            translations: word.translations.iter().map(|t| t.into()).collect(),
            created_at: word.created_at.naive_utc(),
            updated_at: word.updated_at.naive_utc(),
        }
    }
}

impl From<&PgWord> for Word {
    fn from(pg_word: &PgWord) -> Self {
        Word {
            id: pg_word.id,
            value: pg_word.word.clone(),
            kind: (&pg_word.kind).into(),
            tags: pg_word.tags.clone(),
            translations: pg_word.translations.iter().map(|t| t.into()).collect(),
            created_at: DateTime::<Utc>::from_utc(pg_word.created_at.into(), Utc),
            updated_at: DateTime::<Utc>::from_utc(pg_word.updated_at.into(), Utc),
        }
    }
}

impl From<&Row> for PgWord {
    fn from(row: &Row) -> Self {
        PgWord {
            id: row.get(0),
            word: row.get(1),
            kind: row.get(2),
            tags: row.get(3),
            translations: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
        }
    }
}

impl Insertable for PgWord {
    fn columns(&self) -> Vec<&str> {
        vec![
            "id",
            "word",
            "kind",
            "tags",
            "translations",
            "created_at",
            "updated_at",
        ]
    }

    fn values(&self) -> Vec<Value> {
        vec![
            Box::new(self.id),
            Box::new(self.word.clone()),
            Box::new(self.kind),
            Box::new(self.tags.clone()),
            Box::new(self.translations.clone()),
            Box::new(self.created_at),
            Box::new(self.updated_at),
        ]
    }
}

#[derive(Queryable, Debug)]
pub struct PgFolder {
    pub id: uuid::Uuid,
    pub name: String,
    pub words: Vec<uuid::Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, SqlType)]
#[postgres(type_name = "Translation")]
pub struct PgTranslationEntry;
#[derive(Debug, AsExpression, FromSql, ToSql, Clone)]
#[sql_type = "PgTranslationEntry"]
pub struct PgTranslation {
    pub id: uuid::Uuid,
    pub value: String,
}

impl From<&Translation> for PgTranslation {
    fn from(translation: &Translation) -> Self {
        PgTranslation {
            id: translation.id,
            value: translation.value.clone(),
        }
    }
}

impl From<&PgTranslation> for Translation {
    fn from(pg_translation: &PgTranslation) -> Self {
        Translation {
            id: pg_translation.id,
            value: pg_translation.value.clone(),
        }
    }
}

/* impl ToSql<PgTranslationEntry, Pg> for PgTranslation {
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
} */

/* impl FromSql<PgTranslationEntry, Pg> for PgTranslation {
    fn from_sql(
        bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>,
    ) -> deserialize::Result<Self> {
        let values = bytes.unwrap().bytes_from_sql_tuple();
        let id = FromSql::<sql_types::Uuid, Pg>::from_sql(Some(values[0].as_slice()))?;
        let value: String =
            FromSql::<sql_types::VarChar, Pg>::from_sql(Some(values[1].as_slice()))?;
        Ok(PgTranslation { id, value })
    }
} */

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
