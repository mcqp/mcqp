// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.

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