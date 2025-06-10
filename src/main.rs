// mcqp - A CLI tool for sending messages/polls to telegram.
// 
// Copyright (C) 2025 Mohaned Sherhan
// 
// mcqp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// mcqp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with mcqp.  If not, see <https://www.gnu.org/licenses/>.


use clap::Command;

mod command;
mod matches;
mod send;
mod file;
mod sections;
mod parser;
mod log;
mod config;
mod utils;
mod display;
mod check;

#[tokio::main]
async fn main() {
    let arg_matches = Command::new("mcqp")
        .about("A CLI tool for sending messages/polls to telegram.")
        .version("0.1.0-beta.1")
        .author("Mohaned Sherhan")
        .subcommands(command::main())
        .get_matches();
    matches::main(arg_matches).await;
}
