use lib::{
    mongo::{load_options_from_env, MongoDBClient},
    Translation, WordBankClient, WordUpdateOptions,
};

#[test]
fn test_client_update() {
    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();
    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();
    let mut ctx = client.new_context().unwrap();

    let update_options = WordUpdateOptions {
        id: uuid::Uuid::parse_str("f2275b7f-3ad4-4cd5-8d42-0e92f8b4352a").unwrap(),
        word: Some("촉각".to_string()),
        kind: None,
        tags: Some(vec!["Senses".to_string()]),
        translations: Some(vec![Translation::from_value("Sense of touch")]),
    };

    let result = client.update_word(&mut ctx, update_options);
    assert!(result.is_ok());
}
