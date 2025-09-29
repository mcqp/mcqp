// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use pest::iterators::Pairs;
use super::Rule;

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

    /// Parse the poll header.
    pub fn parse_header(&mut self, poll_header_ast: Pairs<'_, Rule>) {
        poll_header_ast 
            .into_iter()
            .filter( |pair| 
                pair.as_rule() == Rule::POLL_HEADER ||
                pair.as_rule() == Rule::MCPOLL_HEADER
            )
            .flat_map( |pair| pair.into_inner() )
            .filter( |inner_pair| 
                inner_pair.as_rule() == Rule::POLL_QUESTION || 
                inner_pair.as_rule() == Rule::MCPOLL_QUESTION
            )
            .take(1)
            .for_each( |inner_pair| {
                self.p = inner_pair.as_span().as_str().to_string();
            });
    }

    /// Parse the poll option.
    pub fn parse_option(&mut self, option_ast: Pairs<'_, Rule>) {
        option_ast
            .into_iter()
            .filter( |pair| pair.as_rule() == Rule::OPTION )
            .flat_map( |pair| pair.into_inner() )
            .filter( |inner_pair| inner_pair.as_rule() == Rule::OPTION_TEXT )
            .take(1)
            .for_each( |inner_pair| {
                self.choices.push(inner_pair.as_span().as_str().to_string());
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

    /// Check the number of opctions.
    pub fn is_options_valid(&self) -> bool {
        if self.choices.len() < 1 || self.choices.len() > 10 {
            return false;
        }
        return true;
    }

    /// Check the poll question length.
    pub fn is_question_valid(&self) -> bool {
        let question_len = self.p.chars().count();
        if question_len < 1 || question_len > 255 {
            return false;
        }
        return true;
    }
}