## MCQP
The `.mcq` file interpreter and a CLI tool for sending messages/polls to **Telegram**.
It is a fast and efficient tool to send bunch of messages/polls to **Telegram** using `.mcq` files,
the `.mcq` file it is a file type use to write the **MCQP** syntax and helps the user to fase write
bunch of messages and polls and send them to **Telegram**. The **MCQP** project came to help the students
or anyone who wants to send a set of questions or make exams on **Telegram**.

## Simple MCQP Syntax Example
```mcq
// config the questions/polls counter to start from 1
config:
    counter = 1

m:(
Hello world from .mcq file!
```rs
fn main() {
  println!("Hello world");
}
```
):endm

q: How are you today? <NOTE: You must be amazing :)>
    Amazing *
    Good
    Bad
    Horrible
```

## Installation
### Linux
- Go to the [Release](https://github.com/mcqp/mcqp/releases) page and download the latest version of **MCQP** for Linux.
- Decompress the file.
- Move the mcqp bin to `/usr/local/bin`:
```sh
$ chmod +x mcqp/bin/mcqp
$ sudo mv mcqp/bin/mcqp /usr/local/bin
```

### Windows
- Download the latest version of **MCQP** from the [Release](https://github.com/mcqp/mcqp/releases) page.
- Extract the contents and move the mcqp folder to your system drive (e.g., C:\mcqp).
- Add the `bin` folder to your system's PATH:
  - Open Environment Variables settings:
    - Press `Win + X`, then select **System**.
    - Click on Advanced system settings > Environment Variables.
  - Under **System variables**, find and select `Path`, then click **Edit**.
  - Add the path to the `mcqp\bin` folder (e.g., C:\mcqp\bin).
  - Click **OK** to save changes.

### From Source
- You need to install **Rust** and **Cargo** on your system.
- Clone the repo:
```sh
$ git clone https://github.com/mcqp/mcqp.git
$ cd mcqp
```
- Build the binary
```sh
$ cargo build --release
```
- Move `mcqp/target/release/mcqp` to your `bin` folder.
- Try to run:
```sh
$ mcqp --version
```

---
> By [Mohaned Sherhan (Mr.x)](https://github.com/Mohaned2023)