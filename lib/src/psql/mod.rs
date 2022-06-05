

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;


use diesel::{PgConnection, Connection};

mod client_pg;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use diesel::prelude::*;

    use crate::WordType;

    use super::models::Word;

    use super::*;
    use super::schema::words::dsl::*;
    

    #[test]
    fn it_works() {
        
        let conn = establish_connection();
        let results = words
            .filter(word.like("S%"))
            .limit(5)
            .load::<Word>(&conn)
            .expect("Error loading words");


    }
}