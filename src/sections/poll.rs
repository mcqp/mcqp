// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;

pub struct Poll {
    /// The poll question
    pub p: String,
    /// The poll choices
    pub choices: Vec<String>,
    /// Is the poll multiple choice
    pub is_mcp: bool
}

impl Poll {
    
    pub fn new() -> Poll {
        return Poll {
            p: String::new(),
            choices: Vec::new(),
            is_mcp: false
        }
    }

    /// Parse the poll question.
    pub fn parse_question(&mut self, line_content: String, len: usize, line_number: usize) {
        let logger = Log::new("poll-parser");
        // Getting the question after the "p:"
        // if the line_content = "p: this is question?"
        // the result will be "this is question?"
        self.p = line_content[len..].trim().to_string();
        // Poll question length must be in range of 1 to 255
        let p_len = self.p.chars().count();
        if p_len < 1 || p_len > 255 {
            logger.error(
                &format!(
                    "The length of the poll question at line {} must be in range of 1 to 255, found {}!", 
                    line_number,
                    p_len
                )
            );
        }
    }

    /// Parse the poll choices.
    pub fn parse_choices(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("poll-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        let choice = line_content.trim().to_string();
                        // The choice length must be in ragne of 1 to 100
                        let choice_len = choice.chars().count();
                        if  choice_len < 1 || choice_len > 100 {
                            logger.error(
                                &format!(
                                    "The choice length at line {} must be in range of 1 to 100, found {}!", 
                                    line_number,
                                    choice_len
                                )
                            );
                        } 
                        self.choices.push(choice);
                    }
                    else {
                        break;
                    }
                } else {
                    logger.error (
                        &format!("Can NOT read line number {}", line_number)
                    );
                }
            } else {
                break;
            }
        }
        // The number of choices must be in range of 1 to 10
        let choices_len = self.choices.len();
        if choices_len < 1 || choices_len > 10 {
            logger.error(
                &format!(
                    "The number of choices at line {} must be in range of 1 to 10, found {}!", 
                    *line_number-1,
                    choices_len
                )
            );
        }
    }
}