/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

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

    pub fn parse_question(&mut self, line_content: String, len: usize) {
        // Getting the question after the "p:"
        // if the line_content = "p: this is question?"
        // the result will be "this is question?"
        self.p = line_content[len..].trim().to_string();
    }

    pub fn parse_choices(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("poll-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        self.choices.push(line_content.trim().to_string());
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
    }
}