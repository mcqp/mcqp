# Contributing to MCQP

Thank you for your interest in contributing to MCQP! We welcome contributions from the community and encourage you to get involved.
Please read through the guidelines below to ensure a smooth process.

## Table of Contents
* [How to Contribute](#how-to-contribute)
* [Code of Conduct](#code-of-conduct)
* [Development Setup](#development-setup)
* [Pull Request Process](#pull-request-process)
* [Testing](#testing)
* [Bug Reports and Feature Requests](#bug-reports-and-feature-requests)

## How to Contribute
1. **Fork the repository**: Start by forking the repository to your own GitHub account.
2. **Clone your fork**: Clone your forked repository to your local machine:
   ```bash
   git clone https://github.com/your-username/mcqp.git
   cd mcqp
   ```
3. **Create a new branch**: It's a good practice to create a new branch for your work. This helps in managing multiple contributions efficiently:
   ```bash
   git checkout -b feature/your-feature
   ```
4. **Make your changes**: Work on the feature or bug fix that you want to implement. Follow the guidelines and ensure your changes are backward compatible with existing functionality.
5. **Commit your changes**: Once your work is done, commit your changes with a clear, concise message describing what was changed.
   ```bash
   git commit -m "Description of the changes made"
   ```
6. **Push your changes**: Push your branch to your forked repository:
   ```bash
   git push origin feature/your-feature
   ```
7. **Submit a Pull Request**: Go to the GitHub page for the repository, switch to your branch, and create a new pull request. Describe your changes clearly and reference any related issues if applicable.


## Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](./CODE_OF_CONDUCT.md).
We are committed to creating a welcoming and inclusive community.


## Code Guideliines
Make sure to read and follow the [Code Guideliines](./CODE_GUIDELINES.md).

## Development Setup
Before contributing, make sure you have the necessary tools set up:
1. **Clone the repository**:
   ```bash
   git clone https://github.com/mcqp/mcqp.git
   cd mcqp
   ```
2. **Install Rust**: Ensure that you have Rust installed. You can install it using [rustup](https://rustup.rs/), which is the recommended way to install Rust. It will also install Cargo (Rust's package manager).
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **Install dependencies**: Rust projects use Cargo to manage dependencies. To install the dependencies, run:
   ```bash
   cargo build
   ```
4. **Run the project**: After installing dependencies, you can run the project locally with:
   ```bash
   cargo run
   ```
5. **Run the tests**: Make sure that all tests pass before submitting any changes:
   ```bash
   cargo test
   ```


## Pull Request Process
1. **Ensure your branch is up-to-date**: Before submitting your pull request, make sure your branch is up to date with the base branch (`main`).
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```
2. **Review your changes**: Double-check your changes, ensuring they are well-documented and that the code follows any existing patterns or structures in the project.
3. **Create a descriptive pull request**: When submitting your pull request, include a clear description of the changes you made, why you made them, and any context that may help reviewers understand your work. If your pull request addresses an issue, reference the issue number (e.g., `Fixes #42`).
4. **Respond to feedback**: If reviewers provide feedback, be sure to address it in a timely manner. We appreciate detailed explanations if changes are requested.
5. **Merge**: Once the pull request is approved and passes all checks, it will be merged into the main branch.


## Testing
We require that all code changes be tested to ensure stability and prevent regressions. Before submitting a pull request, run the tests locally to verify your changes work as expected.

* **Run tests**: To run the project's test suite, use:
  ```bash
  cargo test
  ```
* **Ensure all tests pass**: Please ensure that your changes do not break any existing tests. If you're adding new features, be sure to add tests to verify their correctness.


## Bug Reports and Feature Requests
If you encounter a bug or want to request a new feature, please follow these steps:
1. **Search existing issues**: Before opening a new issue, search through the existing issues to see if someone has already reported the problem or requested the feature.
2. **Create a new issue**: If your issue is unique, create a new issue. Provide a clear and descriptive title and include the following:
   * **Steps to reproduce** (for bugs)
   * **Expected behavior**
   * **Actual behavior**
   * **Environment details** (e.g., operating system, Rust version, dependencies)
3. **Feature requests**: When requesting a new feature, be as detailed as possible about how it could be implemented, why it's necessary, and what benefits it would provide.

---
Thank you for contributing to MCQP! We appreciate your time and effort in helping make this project better.
