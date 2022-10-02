// Web requests
#[macro_use]
extern crate ureq;

use std::io::{self, BufRead};
extern crate dirs;

use nanoserde::{DeJson, SerJson};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, DeJson, SerJson)]
struct ConfigFile {
    chat_id: u32,
    token: String,
}

fn get_config_file_path() -> String {
    dirs::config_dir()
        .expect("Unable to find a config directory. Is your OS supported?")
        .join("tnb.json")
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
            .write_all(
                SerJson::serialize_json(&ConfigFile {
                    chat_id: 123123,
                    token: String::from("Go talk with @BotFather"),
                })
                .as_bytes(),
            )
            .unwrap_or_else(|_| panic!("Cannot write sample config to {config_file_path}"));

        ConfigFileCreationStatus::Created
    }
}

struct Bot {
    chat_id: u32,
    url: String,
}

#[derive(SerJson)]
struct SendMessageRequest {
    chat_id: u32,
    text: String,
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

        let contents =
            std::fs::read_to_string(config_path).expect("Unable to read the configuration file");

        let configuration: ConfigFile = DeJson::deserialize_json(contents.as_str())
            .expect("Unable to parse the configuration file");

        Bot::new(configuration.token, configuration.chat_id)
    }

    fn send_message(&self, text: String) {
        match ureq::post(self.url.as_str())
            .set("Content-Type", "application/json")
            .send_string(
                SerJson::serialize_json(&SendMessageRequest {
                    chat_id: self.chat_id,
                    text,
                })
                .as_str(),
            ) {
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
