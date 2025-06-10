// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;

pub struct Question {
    /// The question string
    pub q: String,
    /// The choices list
    pub choices: Vec<String>,
    /// The correct cohice index
    pub answer: i8,
    /// The question note
    pub note: Option<String>,
}

impl Question {
    pub fn new() -> Question {
        return Question {
            q: String::new(),
            choices: Vec::new(),
            answer: -1,
            note: None
        };
    }

    /// Parse the question string and the note string.
    pub fn parse_question(&mut self, line_content: String, line_number: usize) {
        let logger = Log::new("question-parser");
        // Getting the question and the note after the "q:"
        let q_and_note = line_content[2..].trim();
        if let Some(note_postion) =  q_and_note.find("<NOTE:") {
            self.q = q_and_note[..note_postion].trim().to_string();
            // "<NOTE: this is note>" will be "this is note>"
            let note = q_and_note[note_postion+6..].trim();
            if !note.ends_with(">") {
                // this mean the note is part of the question
                self.q = q_and_note.to_string();
            } else {
                self.note = Some(
                    // remove the end ">" from the note
                    note.strip_suffix(">")
                        .unwrap_or(note)
                        .to_string()
                );
            }
        } else {
            self.q = q_and_note.to_string();
        }
        // Question length must be in range of 1 to 255
        let q_len = self.q.chars().count();
        if q_len > 255 || q_len < 1 {
            logger.error(
                &format!(
                    "The length of the question at line {} must be in range of 1 to 255, found {}!", 
                    line_number,
                    q_len
                )
            );
        }
        // Note length must be in range of 1 to 200
        if self.note.is_some() {
            let note_len = self.note.clone().unwrap().chars().count();
            if note_len < 1 || note_len > 200 {
                logger.error(
                    &format!(
                        "The length of the question note at line {} must be in range of 1 to 200, found {}!",
                        line_number,
                        note_len
                    )
                );
            }
        }
    }

    /// Parse the question choices.
    pub fn parse_choices(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("question-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        let choice = line_content.trim();
                        if choice.ends_with("*") {
                            self.answer = self.choices.len() as i8;
                        }
                        let choice = line_content
                            .strip_suffix("*")
                            .unwrap_or(&line_content)
                            .trim()
                            .to_string();
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
                    logger.error(
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
        if self.answer == -1 {
            logger.error(
                &format!("You must choice the correct option at line {}", line_number)
            );
        }
    }
}