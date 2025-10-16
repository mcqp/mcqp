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
