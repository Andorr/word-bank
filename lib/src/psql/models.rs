use chrono::{DateTime, Utc};
use diesel::Queryable;

use crate::{WordType, Translation};

#[derive(Queryable)]
pub struct Word {
    pub id: uuid::Uuid,
    pub value: String,
    pub kind: WordType,
    pub tags: Vec<String>,
    pub translations: Vec<Translation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable)]
pub struct Folder {
    pub id: uuid::Uuid,
    pub name: String,
    pub words: Vec<uuid::Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}