// Web requests
#[macro_use]
extern crate ureq;

use std::io::{self, BufRead};
extern crate dirs;

// Config file
extern crate ini;
use ini::Ini;
use std::fs::File;
use std::io::prelude::*;

// TODO: make those weird things shorter
fn get_config_file_path() -> String {
    dirs::config_dir()
        .unwrap()
        .join("tnb.ini")
        .to_str()
        .unwrap()
        .to_string()
}

fn create_new_config_file() {
    let mut conf = Ini::new();
    conf.with_section(Some("Bot".to_owned()))
        .set("token", "BOT TOKEN HERE")
        .set("chat_id", "TARGET CHAT ID HERE");

    let mut output = File::create(get_config_file_path()).unwrap();

    // Todo: handle
    output
        .write_all(b"# Warning: Do not share this config file\n")
        .unwrap();
    output
        .write_all(b"# The token will allow anyone to control your bot!\n\n")
        .unwrap();

    match conf.write_to(&mut output) {
        Ok(_) => {
            println!("Created a new config file in:");
            println!("{}", get_config_file_path());
            println!("Better check it out!");
        }
        Err(e) => {
            println!("An error occured while creating the config file:");
            println!("{}", e)
        }
    }
}

struct Bot {
    chat_id: String,
    url: String,
}

impl Bot {
    fn new(token: String, chat_id: String) -> Bot {
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

    fn from_init_file() -> Result<Bot, ()> {
        let conf = Ini::load_from_file(get_config_file_path());

        match conf {
            Ok(conf) => {
                let section = conf.section(Some("Bot".to_owned())).unwrap();
                let token = section.get("token").unwrap();
                let chat_id = section.get("chat_id").unwrap();

                Ok(Bot::new(token.to_string(), chat_id.to_string()))
            }

            Err(_err) => Err(()),
        }
    }

    fn send_message(&self, text: String) {
        match ureq::post(self.url.as_str())
            .set("Content-Type", "application/json")
            .send_json(json!({
            "chat_id": self.chat_id.as_str(),
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
    match Bot::from_init_file() {
        Ok(bot) => {
            bot.read_from_stdin();
        }
        Err(()) => {
            create_new_config_file();
        }
    };
}
