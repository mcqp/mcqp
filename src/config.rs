// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::io::{Read, Write};
use clap::ArgMatches;
use dirs::data_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{display::Display, log::Log, utils};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub chat_id: String,
}

/// The bot API response.
#[derive(Deserialize)]
struct BotResDto<T> {
    result: T
}

/// The bot `getMe` API result.
/// It contains information about the bot.
#[derive(Deserialize)]
pub struct BotResult {
    /// Bot ID
    pub id: i64,
    /// Bot name
    pub first_name: String,
    /// Bot username
    pub username: String,
}

/// The bot `getUpdates` API result.
/// It contains chats and messages information.
#[derive(Deserialize)]
pub struct ChatResult {
    /// The message. It contains:
    /// - `message_id`: The message id
    /// - `from`: The account that send the message
    /// - `chat`: The chat that send the message
    /// - `date`: Sended at
    /// - `text`: The message body
    message: Message
}

/// The message extractor.
#[derive(Deserialize)]
pub struct Message {
    /// The chat information. It contains:
    /// - `id`: The chat ID
    /// - `first_name`: The chat name
    /// - `username`: The chat username
    /// - `type`: The chat type
    pub chat: Chat,
    /// The message body
    pub text: String
}

/// The chat extractor.
#[derive(Deserialize)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub username: String,
    #[serde(rename="type")]
    pub _type: String
}

impl Config {
    pub fn new() -> Config {
        return Config {
            bot_token: String::new(),
            chat_id: String::new()
        };
    }

    /// Set bot token value by the user input. 
    /// Check the bot token by sending a get to 
    /// `https://api.telegram.org/bot<BOT-TOKEN>/getMe`.
    pub async fn set_bot_token(&mut self) {
        let logger = Log::new("set-bot-token");
        let token = utils::input("Enter the token: ");
        let client = Client::new();
        if let Ok(res) = client.get(
                format!("https://api.telegram.org/bot{}/getMe", token)
            )
            .send()
            .await {
            if res.status().is_success() {
                let res_json = res.json::<BotResDto<BotResult>>().await;
                if let Ok(info) = res_json {
                    println!("\n----- Bot Informations:");
                    Display::bot_info(info.result);
                    self.bot_token = token;
                } else {
                    // Can not parse the response to JSON
                    logger.error("Can NOT parse the response!");
                }
            } else {
                // The response is not 200, if it is 400 or 404 or 403 
                // this means the token is invalid.
                logger.error("Invalid bot token!");
            }
        } else {
            // Network error
            logger.error("Network error can NOT send the request!");
        }
    }

    /// Set chat-id based in the bot chats,
    /// Send get to `https://api.telegram.org/bot<BOT-TOKEN>/getUpdates`
    /// to get bot chats.
    pub async fn set_chat_id(&mut self) {
        let logger = Log::new("set-chat-id");
        let client = Client::new();
        if let Ok(res) = client.get(
            format!("https://api.telegram.org/bot{}/getUpdates", self.bot_token)
        )
        .send()
        .await {
            if res.status().is_success() {
                let res_json = res.json::<BotResDto<Vec<ChatResult>>>().await;
                if let Ok(info) = res_json {
                    // show all messages sended to the bot
                    println!("\n----- Chats:");
                    for i in &info.result {
                        Display::bot_chat_info(&i.message);
                        println!("");
                    }
                    // Get the chat-id based on the bot chats
                    let chat_id = utils::input("Based on the chats enter your chat-id: ");
                    if info.result.iter().find(|x| x.message.chat.id.to_string() == chat_id).is_none() {
                        logger.error("Chat id is not in the bot chats!");
                    }
                    self.chat_id = chat_id;
                } else {
                    // Can not parse the response to JSON
                    logger.error("Can NOT parse the response!");
                }
            } else {
                // The response is not 200, if it is 400 or 404 or 403 
                // this means the token is invalid.
                logger.error("Invalid bot token!");
            }
        } else {
            // Network error
            logger.error("Network error can NOT send the request!");
        }
    }

    /// Save configurations to data dir. The config file is not encrypted!
    pub fn save(&self) {
        let logger = Log::new("save-config");
        let data_dir = data_dir().unwrap_or_else(|| logger.error("Can NOT get the data dir!"));
        let config_file = data_dir.join("mcqp/data/config.json");
        if !config_file.exists() {
            let config_file_parent = config_file.parent().unwrap();
            if !config_file_parent.exists() {
                std::fs::create_dir_all(&config_file_parent)
                    .unwrap_or_else(|_| logger.error("Can NOT create the config dir!"));
            }
        }
        let mut file = std::fs::File::create(&config_file)
            .unwrap_or_else(|_| logger.error("Can NOT create the config file!"));
        file.write_all(
            serde_json::to_string(&self)
                .unwrap_or_else(|_| logger.error("Can NOT make json object!"))
                .as_bytes()
        ).unwrap_or_else(|_| logger.error("Can NOT write to the config file!"));
        logger.info("Configurations saved successfully.");
    }

    /// Read configurations from the data dir.
    pub fn get_config(&mut self) {
        let logger = Log::new("get-config");
        let config_file = data_dir()
            .unwrap_or_else(|| logger.error("Can NOT get the data dir!"))
            .join("mcqp/data/config.json");
        if !config_file.exists() {
            logger.error("Configurations NOT found, Please use 'mcqp config' to set the configurations");
        }
        let mut file = std::fs::File::open(config_file)
            .unwrap_or_else(|_| logger.error("Can NOT open the config file!"));
        let mut configs_buf = String::new();
        file
            .read_to_string(&mut configs_buf)
            .unwrap_or_else(|_| logger.error("Can NOT read the config file!"));
        let config: Config = serde_json::from_str(&configs_buf)
            .unwrap_or_else(|_| logger.error("Can NOT parse configs!"));
        self.bot_token = config.bot_token.clone();
        self.chat_id   = config.chat_id.clone();
    }

}

pub async fn main( _: &ArgMatches ) {
    let mut config = Config::new();
    config.set_bot_token().await;
    let _ = utils::input("Please send any text message to the bot and then press enter...");
    config.set_chat_id().await;
    config.save();
}