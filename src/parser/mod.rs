// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

// The sections parsers.
mod poll_parser;

// 3-party packages
use pest::Parser;
use pest_derive::Parser;

// The MCQP modules
use crate::sections::{
    config::Config,
    message::Message,
    question::Question
};
use crate::file::FileReader;

/// The .mcq sections types.
#[derive(PartialEq, Debug)]
pub enum McqpType {
    /// Poll section
    Poll,
    /// Question section
    Question,
    /// Multiple choice poll section
    MCPoll,
    /// Message section
    Message
}

/// The .mcq section tree.
pub struct Mcqp {
    /// The section type.
    pub _type: McqpType,
    /// The poll information.
    pub poll: Option<poll_parser::Poll>,
    /// The question information.
    pub question: Option<Question>,
    /// The message information.
    pub message: Option<Message>
}

/// The .mcq sections tree list.
/// 
/// ### Example:
/// ```
/// McqpList {
///     poll_count: 1,
///     question_count: 0,
///     message_count: 0,
///     config: Config {
///         counter: 20
///     },
///     mcqps: vec![
///         Mcqp {
///             _type: McqpType::Poll,
///             poll: Some(Poll {
///                 p: "A single choice poll".to_string(),
///                 choices: vec![
///                     "One".to_string(),
///                     "Two".to_string(),
///                     "Three".to_string(),
///                 ],
///                 is_mcp: false
///             }),
///             question: None,
///             message: None
///         }
///     ]
/// }
/// ```
pub struct McqpList {
    /// The number of polls in the list.
    pub poll_count: u16,
    /// The number of questions in the list.
    pub question_count: u16,
    /// The number of messages in the list.
    pub message_count: u16,
    /// The list of polls, questions and messages.
    pub mcqps: Vec<Mcqp>,
    /// The MCQP features.
    pub config: Config,
    /// The file reader.
    file_reader: FileReader
}

#[derive(Parser)]
#[grammar = "grammar/mcqp.pest"]
struct MCQPParser;

impl McqpList {
    pub fn new(file_path: std::path::PathBuf) -> Self {
        return McqpList { 
            poll_count: 0, 
            question_count: 0, 
            message_count: 0, 
            mcqps: Vec::new(),
            config: Config::new(),
            file_reader: FileReader::new(file_path)
        };
    }

    /// The main point of the MCQP parser.
    pub fn parse(&mut self) {
        while let Some(line) = &self.file_reader.next_line() {

            // Parse the comment.
            if MCQPParser::parse(Rule::COMMENT, line).is_ok() { continue; }

            // Parse the empty line.
            else if MCQPParser::parse(Rule::EMPTY_LINE, line).is_ok() { continue; }

            // Parse the Poll section.
            else if MCQPParser::parse(Rule::POLL_START, line).is_ok() { self.parse_poll(line); }

        }
    }

    /// The Poll header and the Poll options parser.
    fn parse_poll(&mut self, line: &str) {
        let poll_header_result = MCQPParser::parse(Rule::POLL_HEADER, line);
        if let Ok(poll_header_abt) = poll_header_result {
            let mut poll = poll_parser::Poll::new();
            poll.parse_header(poll_header_abt);
            while let Some(line) = self.file_reader.next_line() {
                if let Ok(option_abt) = MCQPParser::parse(Rule::OPTION, &line) {
                    poll.parse_option(option_abt);
                    if !poll.is_last_option_valid() {
                        // TODO: Error with line and point to the line.
                        todo!("Error with line and point to the line.");
                    }
                    continue;
                }
                break;
            }
            self.file_reader.back_to_previous();
            if !poll.is_options_valid() {
                // TODO: Error with line and point to the line.
                todo!("Error with line and point to the line.");
            }
            if self.config.counter.0 {
                poll.p = format!("[{}] {}", self.config.counter.1, poll.p);
                self.config.counter.1 += 1;
            }
            self.poll_count += 1;
            self.mcqps.push(Mcqp {
                _type: McqpType::Poll,
                poll: Some(poll),
                question: None,
                message: None
            });
        } 
        else if let Err(error) = poll_header_result {
            println!("{:#?}", error);
            // TODO: format the error.
            todo!("format the error.");
        }
    }
}