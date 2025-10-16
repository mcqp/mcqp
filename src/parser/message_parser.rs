// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use telemark::parser::mdv1;
use telemark::parser::enums::MarkdownErrorType;

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

    /// Check the body length.
    pub fn is_valid(&self) -> bool {
        return self.m.len() > 0;
    }
}