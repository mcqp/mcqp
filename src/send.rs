// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use clap::ArgMatches;
use reqwest::Client;
use serde::Serialize;

use crate::file;
use crate::parser;
use crate::log::Log;
use crate::config;
use crate::parser::{McqpAST, Mcqp};

/// The send message DTO.
///
/// See [sendMessage telegram](https://core.telegram.org/bots/api#sendmessage)
#[derive(Serialize)]
struct MessageDto<'a> {
    text: &'a str,
    chat_id: &'a str,
    parse_mode: &'a str
}

/// The send poll DTO.
///
/// See [sendPoll telegram](https://core.telegram.org/bots/api#sendpoll).
#[derive(Serialize)]
struct PollDto<'a> {
    chat_id: &'a str,
    question: &'a str,
    options: &'a Vec<String>,
    is_anonymous: bool,
    #[serde(rename = "type")]
    _type: &'a str,
    allows_multiple_answers: bool,
    correct_option_id: usize,
    explanation: &'a str
}

/// The main function that parse and send the `mcq` file to telegram.
/// The function will run whenever you use:
/// ```shell
/// $ mcqp send <file-path>
/// ```
pub async fn main(command: &ArgMatches) {
    let logger = Log::new("sender");
    let file = command.get_one::<String>("FILE").unwrap();
    let file_state = file::state(file.clone());
    let mut send_config = config::Config::new();
    if file_state == file::FileState::NotFound {
        logger.error("File NOT found!");
    } else if file_state == file::FileState::NotMcqpFile {
        logger.error("File type is NOT .mcq!");
    }
    let mut abstraction_tree = parser::McqpAST::new(
        std::path::PathBuf::new().join(file)
    );
    abstraction_tree.parse();
    send_config.get_config();
    let _ = SendToTelegram::new(abstraction_tree, send_config).send().await;
}

/// Send the mcqp abstraction syntax tree to telegram.
struct SendToTelegram {
    poll_api_url: String,
    message_api_url: String,
    chat_id: String,
    mcq_ast: McqpAST,
    logger: Log,
    client: Client
}
impl SendToTelegram {
    pub fn new(mcq_ast: McqpAST, conf: config::Config) -> Self {
        return Self {
            poll_api_url: format!("https://api.telegram.org/bot{}/sendpoll", conf.bot_token),
            message_api_url: format!("https://api.telegram.org/bot{}/sendMessage", conf.bot_token),
            chat_id: conf.chat_id.clone(),
            mcq_ast,
            logger: Log::new("sender"),
            client: Client::new()
        };
    }

    /// Send .mcq abstraction syntax tree.
    pub async fn send(&self) {
        for section in &self.mcq_ast.mcqps {
            match section._type {
                parser::McqpType::Message  => self.send_message(section).await,
                parser::McqpType::Poll     => self.send_poll(section, false).await,
                parser::McqpType::MCPoll   => self.send_poll(section, true).await,
                parser::McqpType::Question => self.send_question(section).await,
            }
        }
    }

    /// Send the message section.
    async fn send_message(&self, section: &Mcqp) {
        if let Some(message) = &section.message {
            let res_ruslt = self.client.post(&self.message_api_url)
                .json(&MessageDto {
                    chat_id: &self.chat_id,
                    text: &message.m,
                    parse_mode: "Markdown"
                })
                .send()
                .await;
            if let Ok(res) = res_ruslt {
                if !res.status().is_success() {
                    self.logger.error("Can NOT send the message!");
                }
                self.logger.info("message sended successfully");
            } else {
                // Network error
                self.logger.error("Network error, can NOT make a post request!");
            }
        }
    }

    /// Send the poll and mcpoll section.
    async fn send_poll(&self, section: &Mcqp, is_mcpoll: bool) {
        if let Some(poll) = &section.poll {
            let res_ruslt = self.client.post(&self.poll_api_url)
                .json(&PollDto {
                    chat_id: &self.chat_id,
                    question: &poll.question(),
                    options: &poll.choices(),
                    is_anonymous: true,
                    _type: "regular",
                    allows_multiple_answers: is_mcpoll,
                    correct_option_id: 1,
                    explanation: ""
                })
                .send()
                .await;
            if let Ok(res) = res_ruslt {
                if !res.status().is_success() {
                    self.logger.error("Can NOT send the poll!");
                }
                self.logger.info("poll sended successfully");
            } else {
                // Network error
                println!("{:#?}", res_ruslt);
                self.logger.error("Network error, can NOT make a post request!");
            }
        }
    }

    /// Send the question section.
    async fn send_question(&self, section: &Mcqp) {
        if let Some(question) = &section.question {
            let res_ruslt = self.client.post(&self.poll_api_url)
                .json(&PollDto {
                    chat_id: &self.chat_id,
                    question: &question.question(),
                    options: &question.choices(),
                    is_anonymous: true,
                    _type: "quiz",
                    allows_multiple_answers: false,
                    correct_option_id: question.answer() as usize,
                    explanation: &question.note().unwrap_or("".to_string())
                })
                .send()
                .await;
            if let Ok(res) = res_ruslt {
                if !res.status().is_success() {
                    self.logger.error("Can NOT send the question!");
                }
                self.logger.info("question sended successfully");
            } else {
                // Network error
                self.logger.error("Network error, can NOT make a post request!");
            }
        }
    }
}

