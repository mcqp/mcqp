/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use clap::ArgMatches;

pub fn main(command: ArgMatches) {
    match command.subcommand() {
        _ => println!("Hello world")
    }
}