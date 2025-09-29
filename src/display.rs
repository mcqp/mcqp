// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use colored::Colorize;

use crate::config::{BotResult, Message};

pub struct Display;
impl Display {
    /// Display the bot information.
    pub fn bot_info(info: BotResult) {
        println!(
            "ID: {}\nName: {}\nUsername: {}",
            info.id.to_string().green(),
            info.first_name.green(),
            info.username.green()
        );
    }

    /// Display the bot chat information.
    pub fn bot_chat_info(message: &Message) {
        println!(
            "Chat ID: {}\nName: {}\nUsername: {}\nMessage: {}\nChat Type: {}",
            message.chat.id.to_string().green(),
            message.chat.first_name.green(),
            message.chat.username.green(),
            message.text.green(),
            message.chat._type.green()
        )
    }
}

pub struct DisplaySyntaxError;
impl DisplaySyntaxError {
    
    /// Display the syntax error.
    /// 
    /// ## Example:
    /// ```
    /// DisplaySyntaxError::error(
    ///     "Unknown keyword.",
    ///     "Expected a section keyword found unknown keyword",
    ///     &PathBuf::new().join("test.mcq"),
    ///     "the line text.",
    ///     3,
    ///     0
    /// );
    /// ```
    pub fn error(
        msg: &str, 
        position_msg: &str, 
        file_path: &std::path::PathBuf, 
        line: &str, 
        line_number: usize, 
        position: usize
    ) {
        let line_number_width = line_number.to_string().len();
        let position_error_msg = " ".repeat(position) + "^ " + position_msg;
        println!(
            "{}: {}",
            "ERROR".red(),
            msg
        );
        println!(
            " {:line_number_width$} {} {}:{}:{}",
            "",
            "-->".cyan(),
            file_path.display(),
            line_number,
            position
        );
        println!(
            " {:line_number_width$} {}", 
            "",
            "|".cyan()
        );
        println!(
            " {:>line_number_width$} {} {}",
            line_number.to_string().cyan(), 
            "|".cyan(),
            line
        );
        println!(
            " {:line_number_width$} {} {}", 
            "",
            "|".cyan(),
            position_error_msg.red()
        );
        println!("");
    }

    /// Display the error fix using the add-on.
    /// 
    /// ## Example:
    /// ```
    /// DisplaySyntaxError::fix_add(
    ///     "Add '//' to the front of the line.",
    ///     "the line text.",
    ///     "//",
    ///     3,
    ///     0
    /// ); // it will make the line like this "//the line text."
    /// ```
    pub fn fix_add(
        msg: &str,
        line: &str,
        add: &str,
        line_number: usize,
        position: usize
    ) {
        let line_number_width = line_number.to_string().len();
        let plus_with_position = format!("{}{}", " ".repeat(position),"+".repeat(add.chars().count()));
        let mut new_line = line.to_string();
        new_line.insert_str(position, add);
        println!(
            "{}: {}",
            "FIX".green(),
            msg
        );
        println!(
            " {:line_number_width$} {}", 
            "",
            "|".cyan()
        );
        println!(
            " {:>line_number_width$} {} {}",
            line_number.to_string().cyan(), 
            "|".cyan(),
            new_line
        );
        println!(
            " {:line_number_width$} {} {}", 
            "",
            "|".cyan(),
            plus_with_position.green()
        );
        println!("");
    }
}