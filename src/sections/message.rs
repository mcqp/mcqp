// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;

pub struct Message {
    /// The message body
    pub m: String,
}

impl Message {
    pub fn new() -> Message {
        return Message { m: String::new() };
    }

    /// Parse the message body.
    pub fn parse_body(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("message-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next(){
                if let Ok(line_content) = line {
                    if line_content.ends_with("):endm") {
                        break;
                    }
                    self.m += &format!("{}\n", line_content);
                } else {
                    logger.error (
                        &format!("Can NOT read line number {}", line_number)
                    );
                }
            } else {
                break;
            }
        }
        self.m = self.m
            .strip_suffix("\n")
            .unwrap_or(&self.m)
            .to_string();
        if self.m.chars().count() < 1 {
            logger.error(
                &format!("The message length at line {} must be at least one character!", line_number)
            );
        }
    }
}
