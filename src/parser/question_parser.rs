// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use pest::iterators::Pairs;
use super::Rule;

pub struct Question {
    /// The question string
    question: String,
    /// The choices list
    choices: Vec<String>,
    /// The correct cohice index
    answer: i8,
    /// The question note
    note: Option<String>,
}


impl Question {
    pub fn new() -> Self {
        return Self {
            question: String::new(),
            choices: Vec::new(),
            answer: -1,
            note: None
        };
    }

    /// Return a cloned question.
    pub fn question(&self) -> String {
        return self.question.clone();
    }

    /// Return the question length.
    pub fn question_len(&self) -> usize {
        return self.question.chars().count();
    }

    /// Return a cloned choices.
    pub fn choices(&self) -> Vec<String> {
        return self.choices.clone();
    }

    /// Return the answer.
    pub fn answer(&self) -> i8 {
        return self.answer;
    }

    /// Return a option of cloned note.
    pub fn note(&self) -> Option<String> {
        return self.note.clone();
    }

    /// Parse the question header.
    pub fn parse_header(&mut self, question_header_ast: Pairs<'_, Rule>) {
        question_header_ast
            .into_iter()
            .filter( |pair| pair.as_rule() == Rule::QUESTION_HEADER)
            .flat_map( |pair| pair.into_inner() )
            .for_each( |inner_pair| {
                // Parse the questoin.
                if inner_pair.as_rule() == Rule::QUESTION {
                    self.question = inner_pair.as_str().to_string();
                } 
                // Parse the Note block.
                else if inner_pair.as_rule() == Rule::QUESTION_NOTE_BLOCK {
                    inner_pair
                        .into_inner()
                        .filter(|pair| pair.as_rule() == Rule::QUESTION_NOTE)
                        .take(1)
                        .for_each( |note_inner_pair| {
                            self.note = Some(note_inner_pair.as_str().to_string());
                        });
                }
            });
    }

    /// Parse the question option.
    pub fn parse_option(&mut self, option_ast: Pairs<'_, Rule>) {
        option_ast
            .into_iter()
            .filter( |pair| pair.as_rule() == Rule::OPTION )
            .flat_map( |pair| pair.into_inner() )
            .filter( |inner_pair| inner_pair.as_rule() == Rule::OPTION_TEXT )
            .take(1)
            .for_each( |inner_pair| {
                let mut option = inner_pair.as_str().to_string();
                if option.ends_with("*") {
                    option = option
                        .strip_suffix("*")
                        .unwrap_or(&option)
                        .trim()
                        .to_string();
                    // The answer index will be the length of the
                    // current options list, after we add the answer 
                    // we will add this opction.
                    self.answer = self.choices.len() as i8;
                }
                self.choices.push(option);
            });
    }

    /// Check if last option in the opctions list is valid.
    pub fn is_last_option_valid(&self) -> bool {
        if let Some(option) = self.choices.last() {
            let len = option.chars().count();
            if len >= 1 && len <= 100 {
                return true;
            }
        }
        return false;
    }

    /// Check the number of opctions and if there an answer.
    pub fn is_options_valid(&self) -> bool {
        if self.choices.len() < 2 || self.choices.len() > 10  || self.answer == -1 {
            return false;
        }
        return true;
    }

    /// Check the length of the question.
    pub fn is_question_valid(&self) -> bool {
        let question_len = self.question_len();
        if question_len < 1 || question_len > 255 {
            return false;
        }
        return true;
    }

    /// Check if there is note.
    pub fn is_note(&self) -> bool {
        if self.note.is_some() {
            return true;
        }
        return false;
    }

    /// Check the note length.
    pub fn is_note_valid(&self) -> bool {
        if let Some(note) = &self.note {
            let note_len = note.chars().count();
            if note_len < 1 || note_len > 200 {
                return false;
            }
        }
        return true;
    }

    /// Add a counter to the question. It will return `false` if 
    /// the question length so big.
    pub fn add_count(&mut self, counter: usize) -> bool {
        let new_question = format!("{} {}", counter, self.question);
        if new_question.chars().count() > 255 {
            return false;
        }
        self.question = new_question;
        return true;
    }
}