use lib::{Translation, WordBankClient, WordUpdateOptions, mongo::load_options_from_env};


#[test]
fn test_client_update() {
    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();
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