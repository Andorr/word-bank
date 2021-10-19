mod mongo;
mod models;
mod client;

pub use self::client::{WordBankClient, DB};
pub use self::models::{Word, Translation};
pub use self::mongo::{DBOptions};




#[cfg(test)]
mod tests {
    use crate::{DBOptions, Translation, Word, WordBankClient, models::WordType};


    #[test]
    fn client_test() {
        let options = DBOptions {
            uri: "",
            database: "test",
        };
        let client = WordBankClient::from_mongo(options).unwrap();

        let mut word = Word::from_value("먹다");
        word.kind = WordType::Verb;
        let mut translations = vec![Translation::from_value("To eat")];

        let result = client.new_word(&mut word, &mut translations);
        if result.is_err() {
            println!("Failed!");
            return
        }
        println!("{}", result.unwrap());
    }
}
