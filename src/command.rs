/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.


use clap::{arg, Command};

pub fn main() -> [Command; 3] {
    return [
        Command::new("send")
            .arg(arg!(<FILE> "The .mcq file path. (e.g. \"./dir/to/test.mcq\")")),
        Command::new("check")
            .arg(arg!(<FILE> "The .mcq file path. (e.g. \"./dir/to/test.mcq\")")),
        Command::new("config")
    ];
}