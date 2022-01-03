use crate::{
    models::{
        FolderQueryOptions, FolderUpdateOptions, WordQueryOptions, WordType, WordUpdateOptions,
    },
    Folder, PaginationOptions, Translation, Word,
};
use mongodb::bson::serde_helpers::uuid_as_binary;
use mongodb::bson::{self, doc, DateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,
    pub value: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub kind: WordType,
    pub tags: Vec<String>,
    pub translations: Vec<TranslationDBM>,
}

impl WordDBM {
    pub fn update(&mut self, update: &WordUpdateOptions, updated_at: DateTime) {
        if let Some(word) = update.word.clone() {
            self.value = word;
        }
        if let Some(kind) = update.kind {
            self.kind = kind;
        }
        if let Some(tags) = update.tags.clone() {
            self.tags = tags;
        }
        if let Some(translations) = update.translations.clone() {
            self.translations = translations.iter().map(|t| t.into()).collect();
        }
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,
    pub value: String,
}

impl From<WordDBM> for Word {
    fn from(wdbm: WordDBM) -> Self {
        Word {
            id: wdbm._id,
            value: wdbm.value,
            kind: wdbm.kind,
            tags: wdbm.tags,
            translations: wdbm.translations.iter().map(|t| t.into()).collect(),
            created_at: wdbm.created_at.into(),
            updated_at: wdbm.updated_at.into(),
        }
    }
}

impl From<&mut Word> for WordDBM {
    fn from(word: &mut Word) -> Self {
        WordDBM {
            _id: word.id,
            value: word.value.clone(),
            kind: word.kind.clone(),
            tags: word.tags.clone(),
            translations: word.translations.iter().map(|t| t.into()).collect(),
            created_at: word.created_at.into(),
            updated_at: word.updated_at.into(),
        }
    }
}

impl From<&Translation> for TranslationDBM {
    fn from(translation: &Translation) -> Self {
        TranslationDBM {
            _id: translation.id,
            value: translation.value.clone(),
        }
    }
}

impl From<&TranslationDBM> for Translation {
    fn from(translation: &TranslationDBM) -> Self {
        Translation {
            id: translation._id,
            value: translation.value.clone(),
        }
    }
}

#[serde_with::serde_as]
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct FolderDBM {
    #[serde(with = "uuid_as_binary")]
    pub _id: uuid::Uuid,
    pub name: String,

    #[serde_as(as = "Option<bson::Uuid>")]
    pub parent: Option<uuid::Uuid>, // Using bson::Uuid here, due to serialization difficulties with Option<T>.

    #[serde_as(as = "Vec<bson::Uuid>")]
    pub words: Vec<uuid::Uuid>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl FolderDBM {
    pub fn update(&mut self, update: &FolderUpdateOptions, updated_at: DateTime) {
        if let Some(name) = update.name.clone() {
            self.name = name;
        }
        if let Some(parent) = update.parent.clone() {
            self.parent = parent.clone();
            /* if let Some(uuid) = parent {
                self.parent = Some(bson::Uuid::from_bytes(*uuid.as_bytes()));
            } else {
                self.parent = None
            } */
        }
        if let Some(words_to_add) = update.add.clone() {
            self.words.extend(words_to_add);
            self.words.sort_unstable();
            self.words.dedup();
        }
        if let Some(words_to_remove) = update.remove.clone() {
            self.words = self
                .words
                .clone()
                .into_iter()
                .filter(|w_id| !words_to_remove.contains(w_id))
                .collect();
        }

        self.updated_at = updated_at;
    }
}

impl From<FolderDBM> for Folder {
    fn from(folder: FolderDBM) -> Self {
        Folder {
            id: folder._id,
            name: folder.name.clone(),
            parent: if let Some(p) = folder.parent {
                // Some(uuid::Uuid::from_bytes(p.bytes()))
                Some(p)
            } else {
                None
            },
            words: folder.words.clone(),
            created_at: folder.created_at.into(),
            updated_at: folder.updated_at.into(),
        }
    }
}

impl From<&mut Folder> for FolderDBM {
    fn from(folder: &mut Folder) -> Self {
        FolderDBM {
            _id: folder.id,
            name: folder.name.clone(),
            parent: if let Some(p) = folder.parent {
                // Some(bson::Uuid::from_bytes(*p.as_bytes()))
                Some(p)
            } else {
                None
            },
            words: folder.words.clone(),
            created_at: folder.created_at.into(),
            updated_at: folder.updated_at.into(),
        }
    }
}

impl WordQueryOptions {
    pub fn as_query_doc(self) -> Document {
        let mut document = Document::new();

        if let Some(query) = self.query {
            document.insert("$or", vec![
                doc!{"value": bson::Regex { pattern: query.clone(), options: "i".to_string()}},
                doc!{"translations.value": bson::Regex { pattern: query, options: "i".to_string() }},
            ]);
        }
        if let Some(word) = self.word {
            document.insert("value", word);
        }
        if let Some(kind) = self.kind {
            document.insert("kind", kind.to_string());
        }
        if let Some(tags) = self.tags {
            document.insert("tags", tags);
        }

        document
    }

    pub fn as_match_doc(self) -> Document {
        doc! {
            "$match": self.as_query_doc(),
        }
    }
}

impl From<WordQueryOptions> for Document {
    fn from(options: WordQueryOptions) -> Self {
        let mut document = Document::new();

        if let Some(word) = options.word {
            document.insert("value", word);
        }
        if let Some(kind) = options.kind {
            document.insert("kind", kind.to_string());
        }
        if let Some(tags) = options.tags {
            document.insert("tags", tags);
        }

        document
    }
}

impl FolderQueryOptions {
    pub fn as_query_doc(self) -> Document {
        let mut document = Document::new();

        if let Some(query) = self.query {
            document.insert(
                "name",
                bson::Regex {
                    pattern: query.clone(),
                    options: "i".to_string(),
                },
            );
        }
        if let Some(parent) = self.parent {
            document.insert(
                "parent", parent, /* bson::Uuid::parse_str(parent.to_string()).unwrap() */
            );
        }

        if let Some(words) = self.words {
            document.insert(
                "words",
                doc! {
                    "$in": words/* .iter().map(|w| w.to_string()).collect::<Vec<String>>() */,
                },
            );
        }

        document
    }

    pub fn as_match_doc(self) -> Document {
        doc! {
            "$match": self.as_query_doc(),
        }
    }
}

impl PaginationOptions {
    pub fn as_limit_doc(self) -> Document {
        doc! {
            "$limit": self.limit.clone() as u32,
        }
    }
    pub fn as_skip_doc(self) -> Document {
        doc! {
            "$skip": self.skip().clone() as u32,
        }
    }
}

impl WordUpdateOptions {
    pub fn as_set_doc(&self, time: DateTime) -> Document {
        let mut doc = Document::new();

        if let Some(word) = self.word.clone() {
            doc.insert("value", word);
        }
        if let Some(kind) = self.kind {
            doc.insert("kind", kind.to_string());
        }
        if let Some(tags) = self.tags.clone() {
            doc.insert("tags", tags);
        }
        doc.insert("updated_at", time);
        doc
    }
}
