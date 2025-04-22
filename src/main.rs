use core::str;
use std::process::{Command, Output};

fn main() {
    let c = Command::new("docker")
    .args([
        "ps",
        "-a"
    ])
    .output();

    match c {
        Err(_) => print!("Failed to execute command!"),
        Ok(output) => println!("Output: {}", str::from_utf8(output.stdout.as_slice()).unwrap()) 
    }
}
