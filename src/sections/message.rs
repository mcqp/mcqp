// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;
use crate::utils;

pub struct Message {
    /// The message body
    pub m: String,
}

impl Message {
    pub fn new() -> Message {
        return Message { m: String::new() };
    }

    /// Parse the message body and handle the markdown.
    pub fn parse_body(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("message-parser");
        let mut code_block_open = false; 
        let mut star_open = false;
        let mut backtick_open = false;
        let mut underscore_open = false;
        let mut square_bracket_open = false;
        let mut skip_char: u8 = 0;
        loop {
            *line_number += 1;
            if let Some(line) = lines.next(){
                if let Ok(mut line_content) = line {

                    // The end of the message section
                    if line_content.ends_with("):endm") {
                        // if the code block is open just close it by adding ``` in the end of the self.m
                        if code_block_open {
                            self.m += "```";
                        }
                        break;
                    }

                    // The MarkDown parser.
                    for (postion, char) in line_content.chars().enumerate() {
                        // Get the 3 chars every time to check if we entering a code block
                        let code_block_mark = line_content.get(postion..postion+3);
                        if code_block_mark.is_some() {
                            // If the 3 chars is ``` this means it is a code block
                            if code_block_mark.unwrap() == "```" {
                                // Do not parse the 3 chars, this means it will not parse the ``` 
                                skip_char = 3;
                                // Switch the code_block_open if it is open or close
                                // if it is open will be code_block_open = true
                                // if it is close will be code_block_open = false
                                code_block_open = !code_block_open;
                            }
                        }

                        // If the code block is open just continue
                        if code_block_open { continue; }

                        // Parse Telegram Markdown * and ` and _ and []
                        if skip_char != 0 {
                            skip_char -= 1;
                        } else if char == '*' {
                            star_open = !star_open;
                        } else if char == '`' {
                            backtick_open = !backtick_open;
                        } else if char == '_' {
                            underscore_open = !underscore_open;
                        } else if char == '[' {
                            square_bracket_open = true;
                        } else if char == ']' {
                            square_bracket_open = false;
                        }
                    }

                    // If the * or ` or _ or [] is single one, replace it with the escape char
                    // This will work only if the mark is odd, by replacing the last one
                    // it will be even, this means it is closed.
                    if star_open {
                        utils::escape_last(&mut line_content, '*');
                        star_open = false;
                    } else if backtick_open {
                        utils::escape_last(&mut line_content, '`');
                        backtick_open = false;
                    } else if underscore_open {
                        utils::escape_last(&mut line_content, '_');
                        underscore_open = false;
                    } else if square_bracket_open {
                        utils::escape_last(&mut line_content, '[');
                        square_bracket_open = false;
                    }

                    self.m += &format!("{}\n", line_content);
                } else {
                    logger.error (
                        &format!("Can NOT read line number {}", line_number)
                    );
                }
            } else {
                // If we can not read the next line
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
