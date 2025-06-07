/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use std::{fs::File, io::{BufReader, Lines}};

use crate::log::Log;

pub struct Question {
    pub q: String,
    pub choices: Vec<String>,
    pub answer: usize,
    pub note: Option<String>,
}

impl Question {
    pub fn new() -> Question {
        return Question {
            q: String::new(),
            choices: Vec::new(),
            answer: 0,
            note: None
        };
    }

    pub fn parse_question(&mut self, line_content: String) {
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
                    note[..note.chars().count()-1].to_string()
                );
            }
        } else {
            self.q = q_and_note.to_string();
        }
    }

    pub fn parse_choices(&mut self, lines: &mut Lines<BufReader<File>>, line_number: &mut usize) {
        let logger = Log::new("question-parser");
        loop {
            *line_number += 1;
            if let Some(line) = lines.next() {
                if let Ok(line_content) = line {
                    if line_content.starts_with(" ") && line_content.trim().len() > 0 {
                        let choice = line_content.trim();
                        if choice.ends_with("*") {
                            self.answer = self.choices.len();
                        }
                        self.choices.push(
                            line_content
                                .strip_suffix("*")
                                .unwrap_or(&line_content)
                                .trim()
                                .to_string()
                        );
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
    }
}