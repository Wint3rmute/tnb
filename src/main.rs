// Web requests
#[macro_use]
extern crate ureq;

use std::io::{self, BufRead};
extern crate dirs;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static DEFAULT_CONFIG_FILE: &[u8] = b"# Warning: do not share this config file\n\
# The token will allow anyone to control your bot!\n\
TNB_TOKEN=\"BOT TOKEN HERE\" # Edit\n\
TNB_CHAT_ID=0 # Edit\n\
";

// TODO: make those weird things shorter
fn get_config_file_path() -> String {
    dirs::config_dir()
        .expect("Unable to find a config directory. Is your OS supported?")
        .join("tnb.conf")
        .to_str()
        .expect("Config file path is not a valid utf-8 path!")
        .to_string()
}

enum ConfigFileCreationStatus {
    AlreadyExists,
    Created,
}

fn create_new_config_file() -> ConfigFileCreationStatus {
    let config_file_path = get_config_file_path();

    if Path::new(&config_file_path).exists() {
        ConfigFileCreationStatus::AlreadyExists
    } else {
        println!("Creating a new config file");
        File::create(config_file_path.as_str())
            .unwrap_or_else(|_| panic!("Cannot create file {config_file_path}"))
            .write_all(DEFAULT_CONFIG_FILE)
            .unwrap_or_else(|_| panic!("Cannot write sample config to {config_file_path}"));

        ConfigFileCreationStatus::Created
    }
}

struct Bot {
    chat_id: u32,
    url: String,
}

impl Bot {
    fn new(token: String, chat_id: u32) -> Self {
        Bot {
            chat_id,
            url: [
                "https://api.telegram.org/bot",
                token.as_str(),
                "/sendMessage",
            ]
            .join(""),
        }
    }

    fn from_config_file() -> Self {
        let config_path = get_config_file_path();
        match dotenv::from_filename(config_path) {
            Ok(_) => {}
            Err(e) => {
                panic!("Unable to read configuration file: {e}");
            }
        }
        let token = std::env::var("TNB_TOKEN").expect("TNB_TOKEN not found in config file");
        let chat_id: u32 = std::env::var("TNB_CHAT_ID")
            .expect("TNB_CHAT_ID not found in config file")
            .parse()
            .expect("Unable to parse TNB_CHAT_ID");

        Bot::new(token, chat_id)
    }

    fn send_message(&self, text: String) {
        match ureq::post(self.url.as_str())
            .set("Content-Type", "application/json")
            .send_json(json!({
            "chat_id": self.chat_id,
            "text": text.as_str()
            })) {
            Ok(_response) => {}
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }

    fn read_from_stdin(&self) {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            self.send_message(line.unwrap());
        }
    }
}

fn main() {
    match create_new_config_file() {
        ConfigFileCreationStatus::AlreadyExists => Bot::from_config_file().read_from_stdin(),
        ConfigFileCreationStatus::Created => {}
    }
}
