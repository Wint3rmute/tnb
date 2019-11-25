// Web requests
#[macro_use]
extern crate ureq;

// Path related
// use std::fs::create_dir_all;
use std::io::{self, BufRead};
extern crate dirs;

// Config file
extern crate ini;
use ini::Ini;

// fn get_config_dir() -> String {
//     dirs::config_dir().unwrap().to_str().unwrap().to_string()
// }

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
        .set("bot_id", "BOT ID HERE")
        .set("target_chat_id", "TARGET CHAT ID HERE");


    match conf.write_to_file(get_config_file_path()) {
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
    fn new(bot_id: String, chat_id: String) -> Bot {
        Bot {
            chat_id: chat_id,
            url: [
                "https://api.telegram.org/bot",
                bot_id.as_str(),
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
                let bot_id = section.get("bot_id").unwrap();
                let chat_id = section.get("target_chat_id").unwrap();

                Ok(Bot::new(bot_id.to_string(), chat_id.to_string()))
            }

            Err(_err) => Err(()),
        }
    }

    fn send_message(&self, text: String) {

        let resp = ureq::post(self.url.as_str())
        .set("Content-Type", "application/json")
        .send_json(json!({
            "chat_id": self.chat_id.as_str(),
            "text": text.as_str()
            }));
            
        if !resp.ok() {
            match resp.into_string() {
                Ok(response) => {println!("{}", response)},
                Err(err) => println!("{}", err)
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
