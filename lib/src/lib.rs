mod db;

pub use crate::db::{MongoDBClient, DB, DBOptions, Word, Translation};

#[cfg(test)]
mod tests {

}
