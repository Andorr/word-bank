use lib::{DB, DBOptions, MongoDBClient, Translation, Word};

use clap::{Arg, App};



fn main() {
    println!("Hello, world!");

    let app = App::new("WordBank")
        .version("1.0")
        .subcommand(
            App::new("words")
                    .subcommand(
                        App::new("new")
                                .arg(
                                    Arg::with_name("word")
                                        .required(true)
                                        .index(1)
                                )   
                    )
        );
    let matches = app.get_matches();


    println!("{}", matches.subcommand_name().unwrap());
    if let Some(ref matches) = matches.subcommand_matches("words") {
        if let Some(ref matches)  = matches.subcommand_matches("new") {
            let word = matches.value_of("word").unwrap();
            insert_word(String::from(word));
        }
    }
}

fn insert_word(word: String) {
    let uri = std::env::var("WORDBANK_URI").unwrap_or_default();
    let database = std::env::var("WORDBANK_DATABASE").unwrap_or_default();

    let options = DBOptions{
        uri: uri.as_str(),
        database: database.as_str(),
    };

    let client = 
        MongoDBClient::from_options(&options).unwrap();


    let mut word = Word::empty();
    word.value = String::from("한굴");
    let translations: Vec<Translation> = Vec::new(); 
    let id = client.insert_word(&mut word, &translations).unwrap();
    println!("ID: {}", id);
}
