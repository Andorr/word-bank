use lib::{mongo::initialize, mongo::load_options_from_env};

extern crate dotenv;

#[test]
fn initailize_db() {
    dotenv::dotenv().ok();

    let options = load_options_from_env().unwrap();

    let result = initialize(options);
    if result.is_err() {
        println!("Failed to initialize");
        assert!(false);
    }
}
