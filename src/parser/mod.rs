// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

// The sections parsers.
mod poll_parser;
mod question_parser;
mod config_parser;
mod message_parser;

// 3-party packages
use pest::Parser;
use pest_derive::Parser;
use pest::error::{
    InputLocation::Pos,
    ErrorVariant::ParsingError
};

// The MCQP modules
use crate::file::FileReader;
use crate::display::DisplaySyntaxError;
use crate::log::Log;

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
    pub message: Option<message_parser::Message>
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
    /// The file path.
    file_path: std::path::PathBuf,
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
            file_path: file_path.clone(),
            file_reader: FileReader::new(file_path)
        };
    }

    /// The main point of the MCQP parser.
    pub fn parse(&mut self) {
        let logger = Log::new("parser");
        while let Some(line) = &self.file_reader.next_line() {

            // Parse the comment.
            if MCQPParser::parse(Rule::COMMENT, line).is_ok() { continue; }

            // Parse the empty line.
            else if MCQPParser::parse(Rule::EMPTY_LINE, line).is_ok() { continue; }

            // Parse the Poll section.
            else if MCQPParser::parse(Rule::POLL_START, line).is_ok() { 
                self.parse_poll(
                    line, 
                    false, 
                    self.file_reader.get_line_number()
                ); 
            }

            // Parse the Question section.
            else if MCQPParser::parse(Rule::QUESTION_START, line).is_ok() { 
                self.parse_question(
                    line,
                    self.file_reader.get_line_number()
                ); 
            }

            // Parse the Multichoice Poll section.
            else if MCQPParser::parse(Rule::MCPOLL_START, line).is_ok() { 
                self.parse_poll(
                    line, 
                    true,
                    self.file_reader.get_line_number()
                ); 
            }

            // Parse the Config section.
            else if MCQPParser::parse(Rule::CONFIG_START, line).is_ok() { 
                self.parse_config(); 
            }

            // Parse the Message block.
            else if MCQPParser::parse(Rule::MESSAGE_SATRT, line).is_ok() { 
                self.parse_message(&line, self.file_reader.get_line_number());
            }

            // Parse any unknown keyword.
            else { 
                DisplaySyntaxError::error(
                    "Unknown keyword.", 
                    "Expected a section keyword found unknown keyword", 
                    &self.file_path, 
                    line, 
                    self.file_reader.get_line_number(), 
                    0
                );
                DisplaySyntaxError::fix_add(
                    "Add '//' in the front of the line to make it a comment.", 
                    line, 
                    "//", 
                    self.file_reader.get_line_number(),
                    0
                );
                self.exit();
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

    /// The Poll header and the Poll options parser.
    fn parse_poll(&mut self, header_line: &str, is_mcpoll: bool, header_line_number: usize) {
        let poll_header_result = MCQPParser::parse(
            if is_mcpoll { Rule::MCPOLL_HEADER } else { Rule::POLL_HEADER },
            header_line
        );
        if let Ok(poll_header_ast) = poll_header_result {
            let mut poll = poll_parser::Poll::new();
            poll.parse_header(poll_header_ast);
            if !poll.is_question_valid() {
                DisplaySyntaxError::error(
                    "The poll question length is not between 1 to 255 characher.", 
                    &format!(
                        "Expected poll question to be between 1 to 255 characher, found {} characher.", 
                        poll.question().chars().count()
                    ), 
                    &self.file_path, 
                    header_line, 
                    header_line_number, 
                    0
                );
                self.exit();
            }
            // Parsing the opstions.
            while let Some(line) = &self.file_reader.next_line() {
                if let Ok(option_ast) = MCQPParser::parse(Rule::OPTION, line) {
                    poll.parse_option(option_ast);
                    if !poll.is_last_option_valid() {
                        DisplaySyntaxError::error(
                            "The option length is not between 1 to 100 characher.", 
                            &format!(
                                "Expected the option length between 1 to 100 characher, found {} characher.",
                                line.trim().chars().count()
                            ), 
                            &self.file_path,
                            line, 
                            self.file_reader.get_line_number(), 
                            line.chars().take_while( |&c| c == ' ' ).count()
                        );
                        self.exit();
                    }
                    continue;
                }
                break;
            }
            self.file_reader.back_to_previous();
            if !poll.is_options_valid() {
                DisplaySyntaxError::error(
                    "The number of the poll options is not between 1 to 10 option.", 
                    &format!("Expected 1 to 10 options, found {} option.", poll.choices_len()), 
                    &self.file_path, 
                    header_line, 
                    header_line_number, 
                    0
                );
                self.exit();
            }
            if self.config.counter.0 {
                if !poll.add_count(self.config.counter.1) {
                    DisplaySyntaxError::error(
                        "You can not add counter.", 
                        "The length of the question + counter exit 255.", 
                        &self.file_path, 
                        header_line, 
                        header_line_number, 
                        0
                    );
                    self.exit();
                }
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
            let error_position = match error.location {
                Pos(postion) => postion,
                _ => 0
            };
            DisplaySyntaxError::error(
                "The poll question does not exist.", 
                "There is no question for the poll.", 
                &self.file_path, 
                header_line, 
                header_line_number, 
                error_position
            );
            DisplaySyntaxError::fix_add(
                "Add any question to the poll header.", 
                header_line, 
                "Poll question example", 
                header_line_number, 
                error_position
            );
            self.exit();
        }
    }

    /// The Question header and the Question options parser. 
    fn parse_question(&mut self, header_line: &str, header_line_number: usize) {
        let question_header_result = MCQPParser::parse(
            Rule::QUESTION_HEADER, 
            header_line
        );
        if let Ok(question_header_ast) = question_header_result {
            let mut question = question_parser::Question::new();
            question.parse_header(question_header_ast);
            if !question.is_question_valid() {
                DisplaySyntaxError::error(
                    "The question length is not between 1 to 255 characher.", 
                    &format!(
                        "Expected question to be between 1 to 255 characher, found {} characher.", 
                        question.question_len()
                    ), 
                    &self.file_path, 
                    header_line, 
                    header_line_number, 
                    0
                );
                self.exit();
            }
            if question.is_note() {
                if !question.is_note_valid() {
                    DisplaySyntaxError::error(
                        "The length of the note is not between 1 to 200 characher", 
                        "Expected the note length between 1 to 200 characher.",
                        &self.file_path, 
                        header_line, 
                        header_line_number, 
                        0
                    );
                    self.exit();
                }
            }
            while let Some(line) = &self.file_reader.next_line() {
                if let Ok(option_ast) = MCQPParser::parse(Rule::OPTION, line) {
                    question.parse_option(option_ast);
                    if !question.is_last_option_valid() {
                        DisplaySyntaxError::error(
                            "The option length is not between 1 to 100 characher.", 
                            &format!(
                                "Expected the option length between 1 to 100 characher, found {} characher.",
                                line.trim().chars().count()
                            ), 
                            &self.file_path,
                            line, 
                            self.file_reader.get_line_number(), 
                            line.chars().take_while( |&c| c == ' ' ).count()
                        );
                        self.exit();
                    }
                    continue;
                }
                break;
            }
            self.file_reader.back_to_previous();
            if !question.is_options_valid() {
                DisplaySyntaxError::error(
                    "The number of the question options is not between 1 to 10 option or there is no correct answer.", 
                    "Expected 1 to 10 options and a correct answer.", 
                    &self.file_path, 
                    header_line, 
                    header_line_number, 
                    0
                );
                self.exit();
            }
            if self.config.counter.0 {
                if !question.add_count(self.config.counter.1) {
                    DisplaySyntaxError::error(
                        "You can not add counter.", 
                        "The length of the question + counter exit 255.", 
                        &self.file_path, 
                        header_line, 
                        header_line_number, 
                        0
                    );
                    self.exit();
                }
                self.config.counter.1 += 1;
            }
            self.question_count += 1;
            self.mcqps.push(Mcqp { 
                _type: McqpType::Question, 
                poll: None, 
                question: Some(question), 
                message: None 
            });
        } 
        else if let Err(error) = question_header_result {
            let error_position = match error.location {
                Pos(postion) => postion,
                _ => 0
            };
            if let ParsingError { negatives, .. } = error.variant {
                if negatives.len() > 0 {
                    DisplaySyntaxError::error(
                        "Unexpected start of note.", 
                        "The question must be in the first not the note block.", 
                        &self.file_path, 
                        header_line, 
                        header_line_number, 
                        error_position
                    );
                    self.exit();
                }
            }
            DisplaySyntaxError::error(
                "The question does not exist.", 
                "There is no question in the header.", 
                &self.file_path, 
                header_line, 
                header_line_number, 
                error_position
            );
            DisplaySyntaxError::fix_add(
                "Add any question to the header header.", 
                header_line, 
                "Question example",
                header_line_number, 
                error_position
            );
            self.exit();
        }
    }

    /// The Config parser.
    fn parse_config(&mut self) {
        while let Some(line) = &self.file_reader.next_line() {
            if MCQPParser::parse(Rule::CONFIG_FEATURE_START, line).is_ok() {
                let config_ast_result = MCQPParser::parse(Rule::CONFIG_OPSION, line);
                if let Ok(config_ast) = config_ast_result {
                    self.config.parse(config_ast);
                } else if let Err(error) = config_ast_result {
                    let error_position = match error.location {
                        Pos(postion) => postion,
                        _ => 0
                    };
                    if let ParsingError{ positives, ..} = error.variant {
                        positives
                            .iter()
                            .for_each(|&rule| {
                                match rule {
                                    Rule::ASSIGNMENT => {
                                        DisplaySyntaxError::error(
                                            "The config feature missing the assignment operator.", 
                                            "expected an assignment operator, found None.",
                                            &self.file_path, 
                                            line, 
                                            self.file_reader.get_line_number(), 
                                            error_position
                                        );
                                        DisplaySyntaxError::fix_add(
                                            "Add the assignment operator followed by the value to assign the feature value.", 
                                            line, 
                                            " = <FEATURE-VALUE>", 
                                            self.file_reader.get_line_number(), 
                                            error_position
                                        );
                                        self.exit();
                                    },
                                    Rule::CONFIG_COUNTER_VALUE => {
                                        DisplaySyntaxError::error(
                                            "The counter value is missing.", 
                                            "expected a counter value, found None.",
                                            &self.file_path, 
                                            line, 
                                            self.file_reader.get_line_number(), 
                                            error_position
                                        );
                                        DisplaySyntaxError::fix_add(
                                            "Add any number to the counter.", 
                                            line, 
                                            "99", 
                                            self.file_reader.get_line_number(), 
                                            error_position
                                        );
                                        self.exit();
                                    },
                                    _ => { 
                                        DisplaySyntaxError::error(
                                            "Unknown config feature.", 
                                            "expected a config feature, found unknown feature.", 
                                            &self.file_path, 
                                            line, 
                                            self.file_reader.get_line_number(), 
                                            error_position
                                        );
                                        self.exit();
                                    }
                                }
                            });
                    }
                    break;
                }
                continue;
            } 
            break;
        }
        self.file_reader.back_to_previous();
    }

    /// The Message parser.
    fn parse_message(&mut self, message_line: &str, message_line_number: usize) {
        let mut msg = String::new();
        while let Some(line) = &self.file_reader.next_line() {
            if MCQPParser::parse(Rule::MESSAGE_END, line).is_ok() {
                break;
            }
            msg += &format!("{}\n", line);
        }
        let mut message = message_parser::Message::new();
        message.parse(msg);
        if !message.is_valid() {
            DisplaySyntaxError::error(
                "Found message block but there is no message!", 
                "There is no message", 
                &self.file_path, 
                message_line, 
                message_line_number, 
                0
            );
            self.exit();
        }
        self.mcqps.push(Mcqp { 
            _type: McqpType::Message, 
            poll: None, 
            question: None, 
            message: Some(message)
        });
        self.message_count += 1;
    }

    /// Exit the program with `can not parse the file` error.
    fn exit(&self) {
        let logger = Log::new("parser");
        logger.error("Can not parse the file!");
    }
}