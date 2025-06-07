/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use colored::Colorize;

pub struct Log {
    name: String
}

impl Log {

    pub fn new(name: &str) -> Log {
        return Log {
            name: name.to_string()
        };
    }

    pub fn info(&self, message: &str) {
        println!(
            "[{}] @{} - {}",
            "INFO".green(),
            self.name.green(),
            message.green()
        );
    }

    pub fn error(&self, message: &str) -> ! {
        println!(
            "[{}] @{} - {}",
            "ERROR".red(),
            self.name.red(),
            message.red().underline()
        );
        std::process::exit(1);
    }
}