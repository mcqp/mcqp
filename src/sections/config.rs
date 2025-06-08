/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.


use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;

pub struct Config {
    /// The poll/question counter `(is_set: bool, start_from: usize)`
    pub counter: (bool, usize),
}

impl Config {
    /// Create new Config.
    /// Setting:
    /// - `counter` to `(false, 0)`
    pub fn new() -> Config {
        return Config {
            counter: (false, 0)
        };
    }

    /// Parse the config settings.
    /// Search for:
    /// - counter
    pub fn parse_configs(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("config-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        // Parse the counter
                        if line_content.trim().starts_with("counter") {
                            let num_str = line_content
                                .split("=")
                                .last()
                                .unwrap_or("0")
                                .trim();
                            if let Ok(counter) = num_str.parse::<usize>() {
                                self.counter = (true, counter);
                            } else {
                                logger.error (
                                    &format!("Error at line {}, expect number found '{}'", line_number, num_str)
                                );
                            }
                        }
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