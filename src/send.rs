/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use clap::ArgMatches;
use reqwest::Client;
use serde::Serialize;

use crate::file;
use crate::parser;

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

async fn send(abstraction_tree: parser::McqpList) {
    let mut _poll_api_url: String = String::new();
    let mut _message_api_url: String = String::new();
    let mut _chat_id: String = String::new();
    if abstraction_tree.config.is_none() {
        todo!("Get the config from the data dir!");
    } else {
        let config = abstraction_tree.config.unwrap();
        _chat_id = config.chat_id;
        _poll_api_url = format!("https://api.telegram.org/bot{}/sendpoll", config.bot_token);
        _message_api_url = format!("https://api.telegram.org/bot{}/sendMessage", config.bot_token);
    }
    let client = Client::new();
    for section in abstraction_tree.mcqps {
        match section._type {
            parser::McqpType::Message => {
                let message = section.message.unwrap();
                let res_ruslt = client.post(_message_api_url.clone())
                    .json(&MessageDto {
                        chat_id: _chat_id.clone(),
                        text: message.m
                    })
                    .send()
                    .await;
                if let Ok(res) = res_ruslt {
                    if !res.status().is_success() {
                        println!("{:#?}", res);
                        panic!("Can NOT send the message!");
                    }
                } else {
                    panic!("Can NOT make a post request!");
                }
            }
            parser::McqpType::Poll => {
                let poll = section.poll.unwrap();
                let res_ruslt = client.post(_poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: _chat_id.clone(),
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
                        panic!("Can NOT send the poll!");
                    }
                } else {
                    panic!("Can NOT make a post request!");
                }
            }
            parser::McqpType::MCPoll => {
                let poll = section.poll.unwrap();
                let res_ruslt = client.post(_poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: _chat_id.clone(),
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
                        panic!("Can NOT send the poll!");
                    }
                } else {
                    panic!("Can NOT make a post request!");
                }
            }
            parser::McqpType::Question => {
                let question = section.question.unwrap();
                let res_ruslt = client.post(_poll_api_url.clone())
                    .json(&PollDto {
                        chat_id: _chat_id.clone(),
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
                        panic!("Can NOT send the question!");
                    }
                } else {
                    panic!("Can NOT make a post request!");
                }
            }
        }
    }
}

pub async fn main(command: &ArgMatches) {
    let file = command.get_one::<String>("FILE").unwrap();
    let file_state = file::state(file.clone());
    if file_state == file::FileState::NotFound {
        panic!("File NOT found!");
    } else if file_state == file::FileState::NotMcqpFile {
        panic!("File type is NOT .mcq!");
    }
    let mut abstraction_tree = parser::McqpList::new(
        std::path::PathBuf::new().join(file)
    );
    abstraction_tree.parse();
    send(abstraction_tree).await;
}