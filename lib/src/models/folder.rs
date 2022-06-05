use uuid::{self, Uuid};

use chrono::{DateTime, Utc};

use crate::utils::datetime_serializer;
use serde::{Deserialize, Serialize};

use super::word::Word;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: Uuid,
    pub name: String,
    pub parent: Option<Uuid>,
    pub words: Vec<Uuid>,

    #[serde(with = "datetime_serializer")]
    pub created_at: DateTime<Utc>,

    #[serde(with = "datetime_serializer")]
    pub updated_at: DateTime<Utc>,
}

impl Folder {
    pub fn new(name: &str) -> Folder {
        Folder {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent: None,
            words: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update_time(&mut self, time: DateTime<Utc>) {
        self.created_at = time;
        self.updated_at = time;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderContent {
    pub words: Vec<Word>,
    pub folders: Vec<Folder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderQueryOptions {
    pub query: Option<String>,
    pub words: Option<Vec<Uuid>>,
    pub parent: Option<Uuid>,
    pub ids: Option<Vec<Uuid>>,
}

impl FolderQueryOptions {
    pub fn empty() -> FolderQueryOptions {
        FolderQueryOptions {
            query: None,
            words: None,
            parent: None,
            ids: None,
        }
    }

    pub fn ids(mut self, ids: Vec<Uuid>) -> Self {
        self.ids = Some(ids);
        self
    }
}

impl Default for FolderQueryOptions {
    fn default() -> Self {
        FolderQueryOptions::empty()
    }
}

#[derive(Debug, Clone)]
pub struct FolderUpdateOptions {
    pub id: Uuid,
    pub name: Option<String>,
    pub parent: Option<Option<Uuid>>,
    pub add: Option<Vec<Uuid>>,
    pub remove: Option<Vec<Uuid>>,
}
