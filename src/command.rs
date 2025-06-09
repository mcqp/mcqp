// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.


use clap::{arg, Command};

pub fn main() -> [Command; 3] {
    return [
        Command::new("send")
            .about("Parse and then send the .mcq file to telegram.")
            .arg(arg!(<FILE> "The .mcq file path. (e.g. \"./dir/to/test.mcq\")")),
        Command::new("check")
            .about("Check if there any syntax errors.")
            .arg(arg!(<FILE> "The .mcq file path. (e.g. \"./dir/to/test.mcq\")")),
        Command::new("config")
            .about("Configure the bot-token and chat-id.")
    ];
}