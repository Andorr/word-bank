mod config;

use std::{fs, io::Write, str::FromStr};

use config::Config;
use lib::{
    mongo::{DBOptions, MongoDBClient},
    Folder, PaginationOptions, Translation, Word, WordBankClient, WordQueryOptions, WordType,
};

use clap::{App, Arg, ArgMatches};
use serde::{Deserialize, Serialize};

extern crate dotenv;

fn main() {
    let config = config::init_config();
    let app = App::new("WordBank")
        .version("1.0")
        .subcommand(
            App::new("words")
                .subcommand(
                    App::new("new")
                        .arg(Arg::with_name("word").required(true).index(1))
                        .arg(
                            Arg::with_name("translations")
                                .short("t")
                                .long("translation")
                                .multiple(true)
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("kind")
                                .short("k")
                                .long("Kind")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    App::new("list")
                        .arg(Arg::with_name("query").short("q").takes_value(true))
                        .arg(
                            Arg::with_name("limit")
                                .short("l")
                                .long("limit")
                                .default_value("10")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("page")
                                .short("p")
                                .long("page")
                                .default_value("1")
                                .takes_value(true),
                        ),
                )
                .subcommand(App::new("import").arg(Arg::with_name("file").takes_value(true))),
        )
        .subcommand(
            App::new("folders")
                .subcommand(App::new("import").arg(Arg::with_name("file").takes_value(true))),
        );
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("words") {
        if let Some(ref matches) = matches.subcommand_matches("new") {
            let word = matches.value_of("word").unwrap();
            let translations: Vec<&str> = matches.values_of("translations").unwrap().collect();
            let kind = matches.value_of("kind").unwrap_or("NONE");
            insert_word(&config, word, &translations, kind);
        }
        if let Some(ref matches) = matches.subcommand_matches("list") {
            list_words(&config, matches);
        }
        if let Some(ref matches) = matches.subcommand_matches("import") {
            import_words_from_file(&config, matches);
        }
    } else if let Some(ref matches) = matches.subcommand_matches("folders") {
        if let Some(ref matches) = matches.subcommand_matches("import") {
            import_folders_from_file(&config, matches);
        }
    }
}

fn insert_word(cfg: &Config, word: &str, translations: &Vec<&str>, kind: &str) {
    println!("Word: {}", word);
    println!("Translations: {:?}", translations);
    println!("Kind: {}", kind);

    let options = DBOptions {
        uri: cfg.uri.clone(),
        database: cfg.database.clone(),
    };

    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();
    let mut context = client.new_context().unwrap();

    let mut w = Word::from_value(word);
    w.kind = match WordType::from_str(kind) {
        Ok(w) => w,
        Err(_) => {
            println!("Invalid word type: {}", kind);
            return;
        }
    };
    w.translations = translations
        .iter()
        .map(|t| Translation::from_value(*t))
        .collect();

    match client.insert_word(&mut context, &mut w) {
        Ok(id) => {
            println!("Success:\n\tID: {}", id);
        }
        Err(_) => {
            println!("Failure:\n\tWas not able to insert the word!");
        }
    }
}

fn list_words(cfg: &Config, matches: &ArgMatches) {
    let options = DBOptions {
        uri: cfg.uri.clone(),
        database: cfg.database.clone(),
    };

    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();

    let mut context = client.new_context().unwrap();

    let limit = matches
        .value_of("limit")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(10);
    let page = matches
        .value_of("page")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);

    let mut query_options = WordQueryOptions::empty();
    if let Some(query) = matches.value_of("query") {
        query_options.query = Some(query.to_string());
    }

    print!("Loading...");
    std::io::stdout().flush().unwrap();
    let result = client
        .query_words(
            &mut context,
            query_options,
            PaginationOptions::new(limit, page),
        )
        .expect("was not able to list errors");
    print!("\r");

    println!("Count: {}", result.count);
    println!("Total: {}", result.total);
    println!("Page: {}", result.page);
    for word in result.results.iter() {
        println!(
            "- {}: {} {}",
            word.value,
            word.kind.to_string(),
            word.tags.join(",")
        );
        for t in word.translations.iter() {
            println!("\t- {}", t.value);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WordFileType {
    id: String,
    value: String,
    kind: String,
    translations: Vec<String>,
    folders: Vec<String>,
}

fn import_words_from_file(cfg: &Config, matches: &ArgMatches) {
    let file_name = matches.value_of("file").unwrap();
    let content = fs::read_to_string(file_name)
        .expect(format!("was not able to read from file '{}'", file_name).as_str());

    let words: Vec<WordFileType> = serde_json::from_str(content.as_str()).unwrap();
    for w in &words {
        println!("{:?}", w);
    }

    let mut mapped_words: Vec<Word> = (&words)
        .iter()
        .map(|w| {
            let mut word = Word::from_value(w.value.as_str());
            word.id = uuid::Uuid::parse_str(w.id.as_str()).unwrap();
            word.kind = WordType::from_str(w.kind.as_str()).unwrap();
            word.translations = w
                .translations
                .iter()
                .map(|t| Translation {
                    id: uuid::Uuid::new_v4(),
                    value: t.clone(),
                })
                .collect();
            word
        })
        .collect();

    let options = DBOptions {
        uri: cfg.uri.clone(),
        database: cfg.database.clone(),
    };

    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();
    let mut context = client.new_context().unwrap();

    let mut count = 0;
    for w in mapped_words.iter_mut() {
        match client.insert_word(&mut context, w) {
            Ok(_) => count += 1,
            Err(_) => (),
        }
    }

    if let Err(_) = context.commit() {
        println!("An error occured");
        return;
    }

    println!("Added {}/{} words!", count, mapped_words.len());
}

#[derive(Debug, Serialize, Deserialize)]
struct FolderFileType {
    id: String,
    name: String,
    words: Vec<String>,
}

fn import_folders_from_file(cfg: &Config, matches: &ArgMatches) {
    let file_name = matches.value_of("file").unwrap();
    let content = fs::read_to_string(file_name)
        .expect(format!("was not able to read from file '{}'", file_name).as_str());

    let folders: Vec<FolderFileType> = serde_json::from_str(content.as_str()).unwrap();
    for f in &folders {
        println!("{:?}", f);
    }

    let mut mapped_folders: Vec<Folder> = (&folders)
        .iter()
        .map(|f| {
            let mut folders = Folder::new(f.name.as_str());
            folders.id = uuid::Uuid::parse_str(f.id.as_str()).unwrap();
            folders.parent =
                Some(uuid::Uuid::parse_str("61622651-a8d7-43e7-b9fe-b0dfb10fb527").unwrap());
            folders.words = f
                .words
                .iter()
                .map(|w| uuid::Uuid::parse_str(w.as_str()).unwrap())
                .collect();
            folders
        })
        .collect();

    let options = DBOptions {
        uri: cfg.uri.clone(),
        database: cfg.database.clone(),
    };

    let client = WordBankClient::<MongoDBClient>::new(options).unwrap();
    let mut context = client.new_context().unwrap();

    let mut count = 0;
    for f in mapped_folders.iter_mut().rev() {
        match client.insert_folder(&mut context, f) {
            Ok(_) => count += 1,
            Err(_) => (),
        }
    }

    if let Err(_) = context.commit() {
        println!("An error occured");
        return;
    }

    println!("Added {}/{} folders!", count, mapped_folders.len());
}
