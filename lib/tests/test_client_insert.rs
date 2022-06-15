use lib::{
    mongo::{load_options_from_env, MongoDBClient},
    Translation, Word, WordBankClient, WordType,
};

extern crate dotenv;

#[test]
fn test_client_insert_words() {
    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();
    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();

    let mut word = Word::from_value("살다");
    word.kind = WordType::VERB;
    word.translations = vec![Translation::from_value("To live")];

    let mut ctx = client.new_context().unwrap();

    let result = client.insert_word(&mut ctx, &mut word);
    assert!(result.is_ok());
    assert_eq!(word.id, result.unwrap());
}
