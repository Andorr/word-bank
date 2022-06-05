use std::{fs, io::Write};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub uri: String,
    pub database: String,
}

impl Config {
    pub fn default_from_input() -> Config {
        println!("Wordbank config set up:");
        print!("URI: ");
        let _ = std::io::stdout().flush();
        let mut uri = String::new();
        let _ = std::io::stdin().read_line(&mut uri);

        let mut database = String::new();
        print!("Database: ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut database);
        Config {
            uri: uri.as_str().trim().to_string(),
            database: database.as_str().trim().to_string(),
        }
    }
}

pub fn init_config() -> Config {
    let proj_dirs = ProjectDirs::from("", "", "wordbank");
    if proj_dirs.is_none() {
        println!("no project directories found!");
        std::process::exit(1);
    }
    let proj_dirs = proj_dirs.unwrap();
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("wordbank.toml");

    // dbg!(config_dir);
    // dbg!(&config_path);

    let config_file = fs::read_to_string(&config_path);

    match config_file {
        Ok(file) => toml::from_str(&file).unwrap(),
        Err(_) => {
            let cfg = Config::default_from_input();
            let cfg_string = toml::to_string(&(cfg.clone())).expect("was not able to encode TOML");
            let _ = fs::create_dir_all(&config_dir);
            fs::write(config_path, cfg_string).expect("was not able to write config to file");
            cfg
        }
    }
}
