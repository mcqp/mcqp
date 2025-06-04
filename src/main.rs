use clap::Command;

fn main() {
    let matches = Command::new("mcqp")
        .about("")
        .version("0.1.0")
        .author("Mohaned Sherhan")
        .get_matches();
    match matches.subcommand() {
        _ => println!("Hello world")
    }
}
