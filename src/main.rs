extern crate colored;
extern crate exitcode;

use colored::*;
use ctrlc;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Arc;

fn goodbye() {
    clearscreen();
    let goodbye_msg = String::from("The Void hears. Be at peace.");
    println!("{}", goodbye_msg.purple());
    exit(exitcode::OK);
}

fn clearscreen() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    ctrlc::set_handler(move || goodbye()).expect("Error setting ctrlc handler");

    clearscreen();
    let prompt = String::from("Hello! What's troublin' yer noggin'?");
    println!("{}", prompt.purple());

    while running.load(SeqCst) {
        let mut message = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut message).expect(&prompt);
        clearscreen();
    }
}
