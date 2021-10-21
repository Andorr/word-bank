use lib::{DBOptions, PaginationOptions, WordBankClient, WordQueryOptions, WordType};
use std::env;

extern crate dotenv;

#[test]
fn test_client_list_words() {
    // TODO: This is a bad test, it "depends" on existing data to be beneficial

    dotenv::dotenv().ok();

    let db_uri = env::var("WORDBANK_URI").unwrap();
    let database = env::var("WORDBANK_DATABASE").unwrap();

    let options = DBOptions {
        uri: db_uri.as_str(),
        database: database.as_str(),
    };
    let client = WordBankClient::from_mongo(options).unwrap();

    let expected_word = "살다".to_string();

    let result = client.query_words(WordQueryOptions{ 
        word: Some(expected_word.clone()), 
        kind: Some(WordType::VERB), 
        tags: None 
    }, PaginationOptions::new(2, 2));
    assert!(result.is_ok());

    let page_result = result.unwrap();

    assert_eq!(page_result.count, page_result.results.len());
    assert!(page_result.results.len() <= 2);

    page_result.results.iter().for_each(|w| {
        assert!(w.id.len() > 0);
        assert_eq!(expected_word, w.value);
        assert_eq!(WordType::VERB, w.kind);
    });
}