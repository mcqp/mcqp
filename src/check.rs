// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use clap::ArgMatches;

use crate::log::Log;
use crate::file;
use crate::parser;

pub fn main(command: &ArgMatches) {
    let logger = Log::new("checker");
    let file = command.get_one::<String>("FILE").unwrap();
    let file_state = file::state(file.clone());
    if file_state == file::FileState::NotFound {
        logger.error("File NOT found!");
    } else if file_state == file::FileState::NotMcqpFile {
        logger.error("File type is NOT .mcq!");
    }
    let mut abstraction_tree = parser::McqpAST::new(
        std::path::PathBuf::new().join(file)
    );
    abstraction_tree.parse();
    logger.info("Everything looks good.");
}