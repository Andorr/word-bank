use lib::{PaginationOptions, WordBankClient, WordQueryOptions, WordType, mongo::load_options_from_env};

extern crate dotenv;

#[test]
fn test_client_list_words() {
    // TODO: This is a bad test, it "depends" on existing data to be beneficial

    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();
    let client = WordBankClient::from_mongo(options).unwrap();

    let expected_word = "살다".to_string();

    let result = client.query_words(WordQueryOptions{ 
        query: Some(expected_word.clone()),
        word: None, // Some(expected_word.clone()), 
        kind: Some(WordType::VERB), 
        tags: None 
    }, PaginationOptions::new(2, 1));
    assert!(result.is_ok());

    let page_result = result.unwrap();

    assert_eq!(page_result.count, page_result.results.len());
    assert!(page_result.results.len() <= 2);

    page_result.results.iter().for_each(|w| {
        println!("{:?}", w);
        assert_eq!(expected_word, w.value);
        assert_eq!(WordType::VERB, w.kind);
    });
}