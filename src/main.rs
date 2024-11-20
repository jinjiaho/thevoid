use std::io::{stdin,stdout,Write};

fn listen() {
    let mut message=String::new();
    let _=stdout().flush();
    stdin().read_line(&mut message).expect("Speak, the Void is listening.");
    if message == String::from("thank you, goodbye\n") {
        println!("The Void hears you. Be at peace.");
        return;
    }
    println!("mmHmm...");
    listen();
}

fn main() {
    println!("Speak, the Void is listening...");
    listen();
}
