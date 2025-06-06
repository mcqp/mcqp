/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.


use std::{fs::File, io::{BufReader, Lines}};

pub struct Config {
    /// The poll/question counter `(is_set: bool, start_from: usize)`
    pub counter: (bool, usize),
    /// The telegram bot token
    pub bot_token: String,
    /// The send to chat-id
    pub chat_id: String
}

impl Config {
    /// Create new Config.
    /// Setting:
    /// - `counter` to `(false, 0)`
    /// - `bot_token` to `String::new()`
    /// - `chat_id` to `String::new()`
    pub fn new() -> Config {
        return Config {
            counter: (false, 0),
            bot_token: String::new(),
            chat_id: String::new()
        };
    }

    /// Parse the config settings.
    /// Search for:
    /// - counter
    /// - bot-token
    /// - chat-id
    pub fn parse_configs(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        // Parse the counter
                        if line_content.trim().starts_with("counter") {
                            let num_str = line_content
                                .split("=")
                                .last()
                                .unwrap_or("0")
                                .trim();
                            if let Ok(counter) = num_str.parse::<usize>() {
                                self.counter = (true, counter);
                            } else {
                                panic!("Error at line {}, expect number found '{}'", line_number, num_str);
                            }
                        }

                        // Parse the bot-token
                        else if line_content.trim().starts_with("bot-token") {
                            let token = line_content
                                .split("=")
                                .last()
                                .unwrap_or("0:0")
                                .trim();
                            if token.len() < 23 || !token.contains(":") {
                                panic!("Error at line {}, expect a valid telegram token", line_number);
                            }
                            self.bot_token = token.to_string();
                        }

                        // Parse the chat-id
                        else if line_content.trim().starts_with("chat-id") {
                            let id = line_content
                                .split("=")
                                .last()
                                .unwrap_or("")
                                .trim();
                            if id.len() > 5 {
                                self.chat_id = id.to_string();
                            } else {
                                panic!("Error at line {}, expect a valid chat-id", line_number);
                            }
                        }
                    }
                    else {
                        break;
                    }
                } else {
                    panic!("Can NOT read line number {}", line_number);
                }
            } else {
                break;
            }
        }
    }
}