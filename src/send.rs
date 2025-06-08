/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use clap::ArgMatches;
use reqwest::Client;
use serde::Serialize;

use crate::file;
use crate::parser;
use crate::log::Log;
use crate::config;

#[derive(Serialize)]
struct MessageDto {
    text: String,
    chat_id: String
}

#[derive(Serialize)]
struct PollDto {
    chat_id: String,
    question: String,
    options: Vec<String>,
    is_anonymous: bool,
    #[serde(rename = "type")]
    _type: String,
    allows_multiple_answers: bool,
    correct_option_id: usize,
    explanation: String
}

async fn send(abstraction_tree: parser::McqpList, send_config: config::Config) {
    let poll_api_url = format!("https://api.telegram.org/bot{}/sendpoll", send_config.bot_token);
    let message_api_url = format!("https://api.telegram.org/bot{}/sendMessage", send_config.bot_token);
    let logger = Log::new("sender");
    let client = Client::new();
    for section in abstraction_tree.mcqps {
        match section._type {
            parser::McqpType::Message => {
                let message = section.message.unwrap();
                let res_ruslt = client.post(message_api_url.clone())
                    .json(&MessageDto {
                        chat_id: send_config.chat_id.clone(),
                        text: message.m
                    })
                    .send()
                    .await;
                if let Ok(res) = res_ruslt {
                    if !res.status().is_success() {
                        logger.error("Can NOT send the message!");
                    }
                    logger.info("message sended successfully");
                } else {
                    logger.error("Can NOT make a post request!");
                }
            }
            parser::McqpType::Poll => {
                let poll = section.poll.unwrap();
                let res_ruslt = client.post(poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: send_config.chat_id.clone(),
                        question: poll.p,
                        options: poll.choices,
                        is_anonymous: true,
                        _type: "regular".to_string(),
                        allows_multiple_answers: false,
                        correct_option_id: 1,
                        explanation: "".to_string()
                    })
                    .send()
                    .await;
                if let Ok(res) = res_ruslt {
                    if !res.status().is_success() {
                        logger.error("Can NOT send the poll!");
                    }
                    logger.info("poll sended successfully");
                } else {
                    logger.error("Can NOT make a post request!");
                }
            }
            parser::McqpType::MCPoll => {
                let poll = section.poll.unwrap();
                let res_ruslt = client.post(poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: send_config.chat_id.clone(),
                        question: poll.p,
                        options: poll.choices,
                        is_anonymous: true,
                        _type: "regular".to_string(),
                        allows_multiple_answers: true,
                        correct_option_id: 1,
                        explanation: "".to_string()
                    })
                    .send()
                    .await;
                if let Ok(res) = res_ruslt {
                    if !res.status().is_success() {
                        logger.error("Can NOT send the poll!");
                    }
                    logger.info("mc poll sended successfully");
                } else {
                    logger.error("Can NOT make a post request!");
                }
            }
            parser::McqpType::Question => {
                let question = section.question.unwrap();
                let res_ruslt = client.post(poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: send_config.chat_id.clone(),
                        question: question.q,
                        options: question.choices,
                        is_anonymous: true,
                        _type: "quiz".to_string(),
                        allows_multiple_answers: false,
                        correct_option_id: question.answer,
                        explanation: question.note.unwrap_or("".to_string())
                    })
                    .send()
                    .await;
                if let Ok(res) = res_ruslt {
                    if !res.status().is_success() {
                        logger.error("Can NOT send the question!");
                    }
                    logger.info("question sended successfully");
                } else {
                    logger.error("Can NOT make a post request!");
                }
            }
        }
    }
}

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
    let mut abstraction_tree = parser::McqpList::new(
        std::path::PathBuf::new().join(file)
    );
    abstraction_tree.parse();
    send_config.get_config();
    send(abstraction_tree, send_config).await;
}