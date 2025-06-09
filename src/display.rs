// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use colored::Colorize;

use crate::config::{BotResult, Message};

pub struct Display;

impl Display {
    /// Display the bot information.
    pub fn bot_info(info: BotResult) {
        println!(
            "ID: {}\nName: {}\nUsername: {}",
            info.id.to_string().green(),
            info.first_name.green(),
            info.username.green()
        );
    }

    /// Display the bot chat information.
    pub fn bot_chat_info(message: &Message) {
        println!(
            "Chat ID: {}\nName: {}\nUsername: {}\nMessage: {}\nChat Type: {}",
            message.chat.id.to_string().green(),
            message.chat.first_name.green(),
            message.chat.username.green(),
            message.text.green(),
            message.chat._type.green()
        )
    }
}