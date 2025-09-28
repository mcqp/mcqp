// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use pest::iterators::Pairs;
use super::Rule;

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

    /// Parse the question header.
    pub fn parse_header(&mut self, question_header_ast: Pairs<'_, Rule>) {
        question_header_ast
            .into_iter()
            .filter( |pair| pair.as_rule() == Rule::QUESTION_HEADER)
            .flat_map( |pair| pair.into_inner() )
            .for_each( |inner_pair| {
                // Parse the questoin.
                if inner_pair.as_rule() == Rule::QUESTION {
                    self.q = inner_pair.as_str().to_string();
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

    /// Check the number of opctions and if there answer.
    pub fn is_options_valid(&self) -> bool {
        if self.choices.len() < 1 || self.choices.len() > 10  || self.answer == -1 {
            return false;
        }
        return true;
    }
}