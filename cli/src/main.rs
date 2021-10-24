mod config;

use std::str::FromStr;

use config::Config;
use lib::{DBOptions, PaginationOptions, Translation, Word, WordBankClient, WordQueryOptions, WordType};

use clap::{App, Arg, ArgMatches};

extern crate dotenv;

fn main() {
    let config = config::init_config();
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
                                .arg(
                                    Arg::with_name("translations")
                                    .short("t")
                                    .long("translation")
                                    .multiple(true)
                                    .required(true)
                                    .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("kind")
                                    .short("k")
                                    .long("Kind")
                                    .takes_value(true)
                                )
                    )
                    .subcommand(
                        App::new("list")
                                .arg(
                                    Arg::with_name("query")
                                    .short("q")
                                    .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("limit")
                                    .short("l")
                                    .long("limit")
                                    .default_value("10")
                                    .takes_value(true)
                                )
                                .arg(
                                    Arg::with_name("page")
                                    .short("p")
                                    .long("page")
                                    .default_value("1")
                                    .takes_value(true)
                                )
                    )
        );
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("words") {
        if let Some(ref matches)  = matches.subcommand_matches("new") {

            let word = matches.value_of("word").unwrap();
            let translations: Vec<&str> = matches.values_of("translations").unwrap().collect();
            let kind = matches.value_of("kind").unwrap_or("NONE");
            insert_word(&config, word, &translations, kind);
        }
        if let Some(ref matches) = matches.subcommand_matches("list") {
            list_words(&config, matches);
        }
    }
}

fn insert_word(cfg: &Config, word: &str, translations: &Vec<&str>, kind: &str) {
    println!("Word: {}", word);
    println!("Translations: {:?}", translations);
    println!("Kind: {}", kind);

    let options = DBOptions{
        uri: cfg.uri.as_str(),
        database: cfg.database.as_str(),
    };

    let client = WordBankClient::from_mongo(options).unwrap();

    let mut w = Word::from_value(word);
    w.kind = match WordType::from_str(kind) {
        Ok(w) => w,
        Err(_) => {
            println!("Invalid word type: {}", kind);
            return
        }
    };
    w.translations = translations.iter().map(|t| Translation::from_value(*t)).collect();

    match client.insert_word(&mut w) {
        Ok(id) => {
            println!("Success:\n\tID: {}", id);
        },
        Err(_) => {
            println!("Failure:\n\tWas not able to insert the word!");
        }
    }
}

fn list_words(cfg: &Config, matches: &ArgMatches) {
    let options = DBOptions{
        uri: cfg.uri.as_str(),
        database: cfg.database.as_str(),
    };

    let client = WordBankClient::from_mongo(options).unwrap();

    let limit = matches.value_of("limit").unwrap().parse::<usize>().unwrap_or(10);
    let page = matches.value_of("page").unwrap().parse::<usize>().unwrap_or(1);

    let mut query_options = WordQueryOptions::empty();
    if let Some(query) = matches.value_of("query") {
        query_options.query = Some(query.to_string());
    }

    let result = client
        .query_words(query_options, PaginationOptions::new(limit, page))
        .expect("was not able to list errors");

    println!("Count: {}", result.count);
    println!("Total: {}", result.total);
    println!("Page: {}", result.page);
    for word in result.results.iter() {
        println!("- {}: {} {}", word.value, word.kind.to_string(), word.tags.join(","));
        for t in word.translations.iter() {
            println!("\t- {}", t.value);
        }
    }
}