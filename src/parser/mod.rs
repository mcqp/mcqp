// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

// The sections parsers.
mod poll_parser;
mod question_parser;
mod config_parser;

// 3-party packages
use pest::Parser;
use pest_derive::Parser;

// The MCQP modules
use crate::sections::{
    message::Message
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
    pub question: Option<question_parser::Question>,
    /// The message information.
    pub message: Option<Message>
}

#[derive(Parser)]
#[grammar = "grammar/mcqp.pest"]
struct MCQPParser;

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
pub struct McqpAST {
    /// The number of polls in the list.
    pub poll_count: u16,
    /// The number of questions in the list.
    pub question_count: u16,
    /// The number of messages in the list.
    pub message_count: u16,
    /// The list of polls, questions and messages.
    pub mcqps: Vec<Mcqp>,
    /// The MCQP features.
    pub config: config_parser::Config,
    /// The file reader.
    file_reader: FileReader
}

impl McqpAST {
    pub fn new(file_path: std::path::PathBuf) -> Self {
        return McqpAST { 
            poll_count: 0, 
            question_count: 0, 
            message_count: 0, 
            mcqps: Vec::new(),
            config: config_parser::Config::new(),
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
            else if MCQPParser::parse(Rule::POLL_START, line).is_ok() { self.parse_poll(line, false); }

            // Parse the Question section.
            else if MCQPParser::parse(Rule::QUESTION_START, line).is_ok() { self.parse_question(line); }

            // Parse the Multichoice Poll section.
            else if MCQPParser::parse(Rule::MCPOLL_START, line).is_ok() { self.parse_poll(line, true); }

            // Parse the Config section.
            else if MCQPParser::parse(Rule::CONFIG_START, line).is_ok() { self.parse_config(); }

            // Parse the Message block.
            else if MCQPParser::parse(Rule::MESSAGE_SATRT, line).is_ok() { todo!("Parse the message!!"); }

            // Parse any unknown keyword.
            else { todo!("Error with line and point to the line.") }
        }
    }

    /// The Poll header and the Poll options parser.
    fn parse_poll(&mut self, line: &str, is_mcpoll: bool) {
        let poll_header_result = MCQPParser::parse(
            if is_mcpoll { Rule::MCPOLL_HEADER } else { Rule::POLL_HEADER },
            line
        );
        if let Ok(poll_header_ast) = poll_header_result {
            let mut poll = poll_parser::Poll::new();
            poll.parse_header(poll_header_ast);
            while let Some(line) = &self.file_reader.next_line() {
                if let Ok(option_ast) = MCQPParser::parse(Rule::OPTION, line) {
                    poll.parse_option(option_ast);
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
                _type: if is_mcpoll { McqpType::MCPoll } else { McqpType::Poll },
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

    /// The Question header and the Question options parser. 
    fn parse_question(&mut self, line: &str) {
        let question_header_result = MCQPParser::parse(Rule::QUESTION_HEADER, line);
        if let Ok(question_header_ast) = question_header_result {
            let mut question = question_parser::Question::new();
            question.parse_header(question_header_ast);
            while let Some(line) = &self.file_reader.next_line() {
                if let Ok(option_ast) = MCQPParser::parse(Rule::OPTION, line) {
                    question.parse_option(option_ast);
                    if !question.is_last_option_valid() {
                        // TODO: Error with line and point to the line.
                        todo!("Error with line and point to the line.");
                    }
                    continue;
                }
                break;
            }
            self.file_reader.back_to_previous();
            if !question.is_options_valid() {
                // TODO: Error with line and point to the line.
                todo!("Error with line and point to the line.");
            }
            self.question_count += 1;
            self.mcqps.push(Mcqp { 
                _type: McqpType::Question, 
                poll: None, 
                question: Some(question), 
                message: None 
            });
        } else if let Err(error) = question_header_result {
            println!("{:#?}", error);
            // TODO: format the error.
            todo!("format the error.");
        }
    }

    /// The Config parser.
    fn parse_config(&mut self) {
        while let Some(line) = &self.file_reader.next_line() {
            let config_ast_result = MCQPParser::parse(Rule::CONFIG_OPSION, line);
            if let Ok(config_ast) = config_ast_result {
                self.config.parse(config_ast);
            } else if let Err(error) = config_ast_result {
                // TODO: Error with line and point to the line.
                println!("{:#?}", error);
                todo!("Error with line and point to the line.");
                // TODO: If error is not for the config, break the loop.
            }
        }
        self.file_reader.back_to_previous();
    }
}