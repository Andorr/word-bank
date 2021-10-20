mod mongo;
mod models;
mod client;

pub use self::client::{WordBankClient, DB};
pub use self::models::{Word, Translation, WordFilterOptions, PaginationOptions, PageResult};
pub use self::mongo::{DBOptions};




#[cfg(test)]
mod tests {
    use crate::{DBOptions, Translation, Word, WordBankClient, models::{PaginationOptions, WordFilterOptions, WordType}};


    #[test]
    fn client_test() {
        let options = DBOptions {
            uri: "",
            database: "test",
        };
        let client = WordBankClient::from_mongo(options).unwrap();

        let mut word = Word::from_value("살다");
        word.kind = WordType::VERB;
        let mut translations = vec![Translation::from_value("To live")];

        let result = client.new_word(&mut word, &mut translations);
        if result.is_err() {
            println!("Failed!");
            return
        }
        println!("{}", result.unwrap());

        println!("-------------------\n");
        let result = client.list_words(WordFilterOptions::empty(), PaginationOptions::default());
        if result.is_err() {
            println!("Was not able to fetch");
            return
        }
        let page_result = result.unwrap();
        page_result.results.iter().for_each(|w| println!("{:?}", w));
        
        
        println!("-------------------\n");
        let result = client.list_words(WordFilterOptions::empty(), PaginationOptions::new(2, 2));
        if result.is_err() {
            println!("Was not able to fetch");
            return
        }
        let page_result = result.unwrap();
        page_result.results.iter().for_each(|w| println!("{:?}", w));
        
        
        println!("-------------------\n");
        let result = client.list_words(WordFilterOptions{ word: Some("살다".to_string()), kind: None, tags: None }, PaginationOptions::new(2, 2));
        if result.is_err() {
            println!("Was not able to fetch");
            return
        }
        let page_result = result.unwrap();
        page_result.results.iter().for_each(|w| println!("{:?}", w));
    }
}
