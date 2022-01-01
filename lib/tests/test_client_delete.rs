use lib::{Translation, Word, WordBankClient, WordType, mongo::load_options_from_env};

extern crate dotenv;

#[test]
fn test_client_delete_word() {
    // TODO: Make this test more trustworthy and reliable

    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();
    let client = WordBankClient::from_mongo(options).unwrap();

    let mut word = Word::from_value("살다");
    word.kind = WordType::VERB;
    word.translations = vec![Translation::from_value("To live")];

    let mut context = client.new_context().unwrap();

    let result = client.insert_word(&mut context, &mut word);
    assert!(result.is_ok());
    assert_eq!(word.id, result.unwrap());
    
    // TODO: Fetch one the new word to verify its existence
    
    let result = client.delete_word(&mut context, word.id);
    assert!(result.is_ok());

    // TODO: Try to fetch one the new word to verify its nonexistence
}