extern crate colored;

use colored::*;
use std::io::{stdin, stdout, Write};
use std::process::Command;

fn listen(prompt: &ColoredString) {
    let mut message = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut message).expect(prompt);

    clearscreen();
    if message == String::from("thank you for listening\n") {
        let goodbye_msg = String::from("We hear you, love. Be at peace.");
        println!("{}", goodbye_msg.purple());
        return;
    }
    listen(prompt);
}

fn clearscreen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    let prompt = String::from("Hello! What's troublin' yer noggin'?");
    println!("{}", prompt.purple());
    listen(&prompt.purple());
}
