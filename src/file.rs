// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

use std::{ 
    path::PathBuf, 
    fs::File,
    io::{
        BufReader,
        BufRead
    }
};

use crate::log::Log;

/// The mcqp file extension.
const MCQP_FILE_EXTENSION: &str = "mcq";

#[derive(PartialEq)]
pub enum FileState {
    /// file type is .mcq
    McqpFile,
    /// file type is not .mcq
    NotMcqpFile,
    /// file not found
    NotFound,
}

/// Check file state, return `FileState` enum: 
/// - McqpFile
/// - NotMcqpFile
/// - NotFound
/// 
/// ### Example:
/// ```
/// let path = "./dir/to/myfile.mcq".to_string;
/// let file_state = file::state(path);
/// match file_state {
///     file::FileState::NotFound => println!("Not Found file"),
///     file::FileState::NotMcqpFile => println!("Not .mcq file"),
///     file::FileState::McqpFile => println!("File is MCQP.")
/// }
/// ```
pub fn state(file: String) -> FileState {
    let path = std::path::PathBuf::new().join(file);
    if path.is_file() {
        if path
            .extension()
            .expect("Can NOT get the file extension!")
            .to_str()
            .unwrap() == MCQP_FILE_EXTENSION {
            return FileState::McqpFile;
        }
        return FileState::NotMcqpFile;
    }
    return FileState::NotFound;
}



pub struct FileReader {
    /// The file lines.
    lines: Vec<String>,
    /// The number of the current line.
    current_position: usize
}

impl FileReader {

    /// Open the file and read all lines. if file can't be open or read, 
    /// this will print an error and exit the program.
    pub fn new(path: PathBuf) -> Self {
        let logger = Log::new("FileReader");
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(lines) = reader.lines().collect::<Result<Vec<String>,_>>() {
                return FileReader {
                    lines,
                    current_position: 0
                }
            } 
            logger.error("Can't read the file!");
        } else { 
            logger.error("Can't open the file!");
        }
    }

    /// Read the next line in the file.
    /// 
    /// ### Example:
    /// ```
    /// let mut file_reader = FileReader::new(PathBuf::new().join(file_path));
    /// while let Some(line) = file_reader.next_line() {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn next_line(&mut self) -> Option<String> {
        if self.current_position < self.lines.len() {
            let line = self.lines.get(self.current_position).map( |s| s.clone() );
            self.current_position += 1;
            return line;
        }
        return None;
    }

    /// Move the line pointer to the previous line.
    pub fn back_to_previous(&mut self) -> Option<bool> {
        if self.current_position > 0 && self.is_next_line() {
            self.current_position -= 1;
            return Some(true);
        }
        return None;
    }

    /// Get the current line number.
    pub fn get_line_number(&self) -> usize {
        return self.current_position;
    }

    /// Check if there is next line.
    fn is_next_line(&self) -> bool {
        if self.lines.len() == self.current_position {
            return false;
        }
        return true;
    }
}