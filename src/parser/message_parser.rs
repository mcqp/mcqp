// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use telemark::parser::mdv1;
use telemark::parser::enums::MarkdownErrorType;
use telemark::parser::types::MarkdownError;

pub struct Message {
    /// The message body
    pub m: String,
}
impl Message {
    pub fn new() -> Self {
        return Self { m: String::new() };
    }

    /// The markdown parser.
    pub fn parse(&mut self, msg: String) {
        self.m = msg.clone().trim().to_string();
        if let Err(err) = mdv1::parser(&msg) {
            match err.err() {
                MarkdownErrorType::BacktickOpen  => self.m.push('`'),
                MarkdownErrorType::BackticksOpen => self.m += "```",
                MarkdownErrorType::ParenthesesOpen => self.m.push(')'),
                MarkdownErrorType::SquareBracketsOpen => self.m.push(']'),
                MarkdownErrorType::StarOpen => self.m.push('*'),
                MarkdownErrorType::UnderscoreOpen => self.m.push('_'),
            }
        }
    }

    /// Parse the message and throw the errors.
    pub fn parse_with_result(&mut self, msg: String) -> Result<(), MarkdownError> {
        mdv1::parser(&msg)?; // Throw the errors to the up level.
        self.m = msg.clone().trim().to_string();
        return Ok(());
    }

    /// Check the body length.
    pub fn is_valid(&self) -> bool {
        return self.m.len() > 0;
    }
}