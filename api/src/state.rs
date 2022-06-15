use std::sync::Arc;

use lib::{
    mongo::{load_options_from_env, MongoDBClient},
    WordBankClient,
};

#[derive(Clone)]
pub struct State {
    pub client: Arc<WordBankClient<MongoDBClient>>,
}

impl State {
    pub fn new() -> Self {
        let options = load_options_from_env().unwrap();
        let client =
            WordBankClient::<MongoDBClient>::new(options).expect("not able to connect to db");

        Self {
            client: Arc::new(client),
        }
    }
}
