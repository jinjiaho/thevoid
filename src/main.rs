extern crate colored;

use std::io::{stdin,stdout,Write};
use colored::*;

fn listen(prompt: &ColoredString) {
    let mut message=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut message).expect(prompt);

    if message == String::from("thank you, goodbye\n") {
        let goodbye_msg = String::from("The Void hears you. Be at peace.");
        println!("{}", goodbye_msg.purple());
        return;
    }
    let mm_hmm = String::from("mmHmm...");
    println!("{}", mm_hmm.purple());
    listen(prompt);
}

fn main() {
    let prompt = String::from("Speak, the Void is listening...");
    println!("{}", prompt.purple());
    listen(&prompt.purple());
}
