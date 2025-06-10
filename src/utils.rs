// This file is part of mcqp project, licensed under the GPL v3.
// See the LICENSE file for full license details.


/// Read a string from standard input.
/// 
/// ### Example
/// ```
/// let input = utilities::input("Enter your name: ");
/// if input == "Mohaned" {
///     println!("Hello Mohaned");
/// } else {
///     println!("hi {}", input);
/// }
/// ```
pub fn input(message: &str) -> String {
    use std::io::Write;
    print!("{}", message);
    std::io::stdout().flush().expect("Flush Error!");
    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Read line Error!");
    return line.trim().to_owned();
}

/// Escaping the last char matches in the string by adding `\` in 
/// front of it.
/// 
/// ### Example:
/// ```
/// let mut string = "the *text* * ok".to_string();
/// utils::escape_last(&mut string, '*');
/// assert_eq!(
///     string,
///     "the *text* \\* ok".to_string()
/// )
/// ```
pub fn escape_last(string: &mut String, mark: char) {
    let found_result = string.rfind(mark);
    if found_result.is_some() {
        let lest_mark_postion = found_result.unwrap();
        string.replace_range(
            lest_mark_postion..lest_mark_postion+1, 
            &format!("\\{}", mark)
        );
    }
}