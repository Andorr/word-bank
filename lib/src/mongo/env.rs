use crate::{mongo::DBOptions};

use std::env;

pub fn load_options_from_env() -> Result<DBOptions, String> {
    
    let uri = match env::var("WORDBANK_DB_URI") {
        Ok(uri) => uri.clone(),
        Err(_) => return Err(String::from("missing environment variable: WORDBANK_DB_URI")),
    };

    let database = match env::var("WORDBANK_DB_DATABASE") {
        Ok(database) => database.clone(),
        Err(_) => return Err(String::from("missing environment variable: WORDBANK_DB_DATABASE")),
    };

    Ok(DBOptions { uri: uri, database: database })
}