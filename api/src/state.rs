use std::sync::Arc;

use lib::{WordBankClient, mongo::load_options_from_env};

#[derive(Clone)]
pub struct State {

    pub client: Arc<WordBankClient>,
}

impl State {
    pub fn new() -> Self {
        let options = load_options_from_env().unwrap();
        let client = WordBankClient::from_mongo(options).expect("not able to connect to db");

        Self {
            client: Arc::new(client),
        }
    }
}