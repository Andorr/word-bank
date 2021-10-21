use std::env;

use lib::{DBOptions, initialize};

extern crate dotenv;

#[test]
fn initailize_db() {

    dotenv::dotenv().ok();

    let db_uri = env::var("WORDBANK_URI").unwrap();
    let database = env::var("WORDBANK_DATABASE").unwrap();

    let options = DBOptions {
        uri: db_uri.as_str(),
        database: database.as_str(),
    };

    let result = initialize(options);
    if result.is_err() {
        println!("Failed to initialize");
        assert!(false);
    }
}