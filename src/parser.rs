/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use std::{fs::File, io::{BufRead, BufReader}};

use crate::sections::{
    config::Config,
    poll::Poll,
    message::Message,
    question::Question
};
use crate::log::Log;

#[derive(PartialEq, Debug)]
pub enum McqpType {
    /// Send as poll
    Poll,
    /// Send as question
    Question,
    /// Send as multiple choice poll
    MCPoll,
    /// Send as message
    Message
}

pub struct Mcqp {
    pub _type: McqpType,
    pub poll: Option<Poll>,
    pub question: Option<Question>,
    pub message: Option<Message>
}

pub struct McqpList {
    pub poll_count: u16,
    pub question_count: u16,
    pub message_count: u16,
    pub file_path: std::path::PathBuf,
    pub mcqps: Vec<Mcqp>,
    pub config: Option<Config>
}

impl McqpList {
    pub fn new(file_path: std::path::PathBuf) -> McqpList {
        return McqpList { 
            poll_count: 0, 
            question_count: 0, 
            message_count: 0, 
            file_path: file_path,
            mcqps: Vec::new(),
            config: None
        };
    }

    pub fn parse(&mut self) {
        let file = File::open(self.file_path.clone())
            .expect("Can NOT open the file!");
        let reader = BufReader::new(file);
        let mut lines: std::io::Lines<BufReader<File>> = reader.lines();
        let mut line_number: usize = 0;
        let logger = Log::new("parser");
        loop {
            line_number += 1;
            if let Some( line ) = lines.next() {
                if let Ok(line_content ) = line {
                    
                    // Parse new line and space
                    if line_content.trim().len() == 0 { continue; }

                    // Parse the poll section 
                    else if line_content.starts_with("p:") {
                        let mut poll = Poll::new();
                        poll.parse_question(line_content.clone(), 2);
                        poll.parse_choices(&mut lines, &mut line_number);
                        self.poll_count += 1;
                        self.mcqps.push(Mcqp {
                            _type: McqpType::Poll,
                            poll: Some(poll),
                            question: None,
                            message: None
                        });
                    }

                    // Parse the question section
                    else if line_content.starts_with("q:") { 
                        let mut question = Question::new();
                        question.parse_question(line_content.clone());
                        question.parse_choices(&mut lines, &mut line_number);
                        self.question_count += 1;
                        self.mcqps.push(Mcqp {
                            _type: McqpType::Question,
                            poll: None,
                            question: Some(question),
                            message: None,
                        });
                    }

                    // Parse the multiple choice poll section
                    else if line_content.starts_with("mcp:") { 
                        let mut mcpoll = Poll::new();
                        mcpoll.parse_question(line_content.clone(), 4);
                        mcpoll.is_mcp = true;
                        mcpoll.parse_choices(&mut lines, &mut line_number);
                        self.poll_count += 1;
                        self.mcqps.push(Mcqp {
                            _type: McqpType::MCPoll,
                            poll: Some(mcpoll),
                            question: None,
                            message: None
                        });
                    }

                    // Parse the message section
                    else if line_content.starts_with("m:(") { 
                        let mut message = Message::new();
                        message.parse_body(&mut lines, &mut line_number);
                        self.message_count += 1;
                        self.mcqps.push(Mcqp {
                            _type: McqpType::Message,
                            poll: None,
                            question: None,
                            message: Some(message)
                        });
                    }

                    // Parse the config section
                    else if line_content.starts_with("config:") { 
                        let mut config = Config::new();
                        config.parse_configs(&mut lines, &mut line_number);
                        self.config = Some(config);
                    }

                    // Parse the comments
                    else if line_content.trim().starts_with("//") { continue; }

                    else { 
                        logger.error(
                            &format!("Error in line {}", line_number)
                        );
                    }

                } else {
                    logger.error(
                        &format!("Can NOT read line number {}", line_number)
                    );
                }
            } else {
                break;
            }
        }
        logger.info(
            &format!(
                "found {}/poll and {}/question and {}/message", 
                self.poll_count,
                self.question_count,
                self.message_count
            )
        );
    }
}

