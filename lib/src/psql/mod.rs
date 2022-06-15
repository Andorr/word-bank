mod context;
pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::{Connection, PgConnection};

mod client_pg;

pub use client_pg::PgDBClient;
pub use context::PgContext;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use diesel::prelude::*;

    use super::models::{Translation, WordType};

    use super::models::Word;

    use super::schema::words::dsl::*;
    use super::*;

    fn create_word(conn: &PgConnection) -> Word {
        use super::schema::words::dsl::*;

        let time = Utc::now();

        let new_word = Word {
            id: uuid::Uuid::new_v4(),
            word: "나뉘다".to_string(),
            kind: WordType::VERB,
            tags: vec![],
            translations: vec![
                Translation {
                    id: uuid::Uuid::new_v4(),
                    value: "To Divide".to_string(),
                },
                Translation {
                    id: uuid::Uuid::new_v4(),
                    value: "To Share".to_string(),
                },
            ],
            created_at: time.naive_utc(),
            updated_at: time.naive_utc(),
        };

        diesel::insert_into(words)
            .values(new_word)
            .get_result(conn)
            .expect("Error saving new word")
    }

    #[test]
    fn it_works() {
        let conn = establish_connection();
        let mut results: Vec<Word> = words
            // .filter(word.like("S%"))
            .limit(5)
            .load(&conn)
            .expect("Error loading words");

        println!("Words: {:?}", results);
        create_word(&conn);

        results = words
            // .filter(word.like("S%"))
            .limit(5)
            .load(&conn)
            .expect("Error loading words");

        println!("Words: {:?}", results);
    }
}