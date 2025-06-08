/// This file is part of mcqp project, licensed under the GPL v3.
/// See the LICENSE file for full license details.

use clap::ArgMatches;

pub async fn main(commands: ArgMatches) {
    match commands.subcommand() {
        Some(("send", command)) => crate::send::main(command).await,
        Some(("config", command)) => crate::config::main(command).await,
        Some(("check", command)) => crate::check::main(command),
        _ => println!("Hello world")
    }
}