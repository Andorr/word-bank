use lib::{DBOptions, WordBankClient, WordType, Word, Translation};
use std::env;

extern crate dotenv;

#[test]
fn test_client_insert_words() {
    dotenv::dotenv().ok();

    let db_uri = env::var("WORDBANK_URI").unwrap();
    let database = env::var("WORDBANK_DATABASE").unwrap();

    let options = DBOptions {
        uri: db_uri.as_str(),
        database: database.as_str(),
    };
    let client = WordBankClient::from_mongo(options).unwrap();

    let mut word = Word::from_value("살다");
    word.kind = WordType::VERB;
    word.translations = vec![Translation::from_value("To live")];

    let result = client.new_word(&mut word);
    assert!(result.is_ok());
    assert_eq!(word.id, result.unwrap());
}