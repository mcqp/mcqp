# Code Guidelines for MCQP
To maintain consistency, readability, and quality in our codebase, we follow these coding standards and guidelines for contributing to MCQP.
Please take the time to read and adhere to these guidelines when contributing to the project.


## 1. Naming Conventions
Consistency in naming is crucial for readability and understanding the codebase. The following naming conventions should be followed:

### 1.1 Variables and Function Names
* Use `snake_case` for variables and function names.
  ```rust
  let my_variable = 42;
  fn calculate_area(radius: f64) -> f64 { ... }
  ```

### 1.2 Constants and Static Variables
* Use `UPPER_SNAKE_CASE` for constants and static variables.
  ```rust
  const MAX_RETRIES: u32 = 3;
  static HELLO_WORLD: &str = "Hello, world!";
  ```

### 1.3 Structs, Enums, and Traits
* Use `CamelCase` for struct, enum, and trait names.
  ```rust
  struct UserProfile { ... }
  enum PaymentMethod { CreditCard, PayPal }
  trait Renderable { ... }
  ```


## 2. Error Handling
Rust’s powerful error handling mechanism is one of its strengths. We strongly encourage using the `Result` and `Option` types for handling errors.

### 2.1 Use `Result` and `Option`
* For **recoverable errors**, use `Result`.
* For **optional values**, use `Option`.
  ```rust
  fn read_file(path: &str) -> Result<String, std::io::Error> {
      let mut file = std::fs::File::open(path)?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      Ok(contents)
  }
  ```

### 2.2 Propagate Errors with `?`
* Use the `?` operator to propagate errors instead of manually unwrapping or handling them.
  ```rust
  fn get_user_profile(id: u32) -> Result<UserProfile, Error> {
      let user = fetch_user_from_db(id)?;
      Ok(user)
  }
  ```


## 3. Avoid `unsafe`
Rust’s primary strength lies in its safety features. We should minimize the use of `unsafe` code as much as possible.
* **Only use `unsafe` when absolutely necessary**.
* When `unsafe` code is required, ensure it is well-documented with explanations for why it’s used and how it guarantees safety.


## 4. Documentation
Document all **public** functions, structs, and enums. This improves code readability and provides helpful context for others
working with your code.

* **Use Rust’s documentation comments** (`///`) to document functions, types, and modules.
* **Provide examples** for functions where possible.

### Example:
````rust
/// Calculates the area of a circle given the radius.
///
/// # Arguments
/// * `radius` - The radius of the circle
///
/// # Returns
/// The area of the circle.
///
/// # Example
/// ```
/// let area = calculate_area(3.0);
/// assert_eq!(area, 28.27);
/// ```
fn calculate_area(radius: f64) -> f64 { 
    3.14159 * radius * radius 
}
````


## 5. Unit Testing
We encourage writing **unit tests** for all new features or bug fixes to ensure the reliability of the project.

### 5.1 Writing Tests
* Place unit tests in a `tests` module at the bottom of the file, or in a separate file under the `tests/` directory.
* Use `#[cfg(test)]` to conditionally include tests.

### Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_area() {
        assert_eq!(calculate_area(3.0), 28.27);
    }
}
```

## 6. Code Structure and Organization
### 6.1 Modules
* Use **snake_case** for module names and file names.
* Group related functionality into modules and keep module files small and focused.
  
  Example:
  ```rust
  mod user_profile; // user_profile.rs file
  ```

### 6.2 Avoid Long Functions
* Break up functions that are too long into smaller, more manageable ones. A function should ideally do one thing and do it well.

### 6.3 Keep Logic Simple
* Avoid deeply nested logic or overly complex expressions. Try to break down the problem into simpler steps.


## 7. Comments
### 7.1 Use Comments Wisely
* **Explain why**, not **what**: Focus comments on explaining *why* certain decisions are made or why something is being done a particular way, rather than stating the obvious.

  Example of good comment:
  ```rust
  // We use `unsafe` here because we need direct access to the hardware register.
  let register = unsafe { read_register(0xFF) };
  ```

### 7.2 Documentation Comments
* Use **documentation comments** (`///`) for public APIs and modules. This allows tools like `rustdoc` to generate API documentation automatically.


## 8. Use Idiomatic Rust
Follow the [Rust Book](https://doc.rust-lang.org/book/) and [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for idiomatic Rust practices. A few quick points:
* Prefer **`iter()`**, **`map()`**, and other iterator combinators over manual loops.
* Use **`match`** expressions when handling multiple cases.
* Use **`Option`** and **`Result`** instead of `null` or error codes.


## 9. Version Compatibility
Make sure that the code is compatible with the **minimum supported Rust version (MSRV)**, and update dependencies when necessary. Check the compatibility by running:
```bash
cargo check
```

---
By following these guidelines, you help maintain the quality and consistency of the codebase. Thank you for contributing to MCQP!
