use std::env;

use lib::{DBOptions, Translation, WordBankClient, WordUpdateOptions};


#[test]
fn test_client_update() {
    dotenv::dotenv().ok();

    let db_uri = env::var("WORDBANK_URI").unwrap();
    let database = env::var("WORDBANK_DATABASE").unwrap();

    let options = DBOptions {
        uri: db_uri.as_str(),
        database: database.as_str(),
    };
    let client = WordBankClient::from_mongo(options).unwrap();
    
    let update_options = WordUpdateOptions {
        id: uuid::Uuid::parse_str("f2275b7f-3ad4-4cd5-8d42-0e92f8b4352a").unwrap(),
        word: Some("촉각".to_string()),
        kind: None,
        tags: Some(vec!["Senses".to_string()]),
        translations: Some(vec![
            Translation::from_value("Sense of touch")
        ]),
    };

    let result = client.update_word(update_options);
    assert!(result.is_ok());
}