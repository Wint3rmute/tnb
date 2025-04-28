extern crate ureq;

use std::io::{self, BufRead};
extern crate dirs;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY_ADDRESS: &str = env!("CARGO_PKG_REPOSITORY");

enum ConfigFileCreationStatus {
    AlreadyExists,
    Created,
}

#[derive(Serialize, Deserialize)]
struct ConfigurationFile {
    chat_id: u32,
    token: String,
}

impl ConfigurationFile {
    fn get_path() -> String {
        dirs::config_dir()
            .expect("Unable to find a config directory. Is your OS supported?")
            .join("tnb.json")
            .to_str()
            .expect("Config file path is not a valid utf-8 path!")
            .to_string()
    }

    fn create_new() -> ConfigFileCreationStatus {
        let config_file_path = Self::get_path();

        if Path::new(&config_file_path).exists() {
            return ConfigFileCreationStatus::AlreadyExists;
        }

        println!("Creating a new config file at {config_file_path}");
        let default_configuration = serde_json::to_string(&ConfigurationFile {
                    chat_id: 0,
                    token: String::from("Go talk with @BotFather"),
                })
                .unwrap_or_else(|error| {
                    panic!("Unable to write sample config file, but this is most probably developer's fault: {error}")
                });

        fs::write(config_file_path.as_str(), default_configuration.as_bytes()).unwrap_or_else(
            |error| panic!("Cannot write sample config to {config_file_path}: {error}"),
        );

        ConfigFileCreationStatus::Created
    }

    fn load() -> Self {
        let config_path = Self::get_path();

        let contents = fs::read_to_string(config_path)
            .unwrap_or_else(|error| panic!("Unable to read the configuration file: {error}"));

        serde_json::from_str(contents.as_str())
            .unwrap_or_else(|error| panic!("Unable to parse the configuration file: {error}"))
    }
}

struct Bot {
    chat_id: u32,
    send_message_endpoint: String,
}

impl Bot {
    fn from_configuration_file() -> Self {
        let configuration = ConfigurationFile::load();
        Bot {
            chat_id: configuration.chat_id,
            send_message_endpoint: [
                "https://api.telegram.org/bot",
                configuration.token.as_str(),
                "/sendMessage",
            ]
            .join(""),
        }
    }

    fn send_message(&self, text: String) {
        match ureq::post(self.send_message_endpoint.as_str())
            .header("Content-Type", "application/json")
            .send_json(json!({
            "chat_id": self.chat_id,
            "text": text.as_str()
            })) {
            Ok(_response) => {}
            Err(error) => {
                eprintln!("{error}");
            }
        }
    }

    fn forward_from_stdin(&self) {
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            self.send_message(line.unwrap());
        }
    }
}

fn main() {
    if std::env::args().len() > 1 {
        println!("tnb - Telegram Notification Bot, version {VERSION}");
        println!();

        println!("This program is meant to be run without any arguments/flags and take input from stdin or a pipe.");

        let conf_file_path = ConfigurationFile::get_path();
        println!("If you're having issues with sending messages, verify your configuration file: {conf_file_path}");
        println!();

        println!("For more help, refer to project's README.md: {REPOSITORY_ADDRESS}");
        println!();
        println!("Have a peaceful day!");

        std::process::exit(1);
    }

    match ConfigurationFile::create_new() {
        ConfigFileCreationStatus::Created => {}
        ConfigFileCreationStatus::AlreadyExists => {
            Bot::from_configuration_file().forward_from_stdin();
        }
    }
}
