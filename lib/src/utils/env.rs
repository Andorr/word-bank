use std::env;

use dotenv::dotenv;

#[derive(Debug, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

pub fn get_environment() -> Environment {
    let environ = env::var("ENVIRONMENT")
    .unwrap_or(String::from("Development"))
    .to_lowercase();
    
    match environ.as_str() {
        "production" => Environment::Production,
        _ => Environment::Development
    }
}

pub fn load_env() -> Result<(), ()> {
    let environ = get_environment();
    
    if environ != Environment::Production {
        match dotenv() {
            Ok(_) => (),
            Err(_) => return Err(())
        };
    }

    Ok(())
}