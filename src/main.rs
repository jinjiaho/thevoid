extern crate colored;

use colored::*;
use ctrlc;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn exit(r: &Arc<AtomicBool>) {
  clearscreen();
  let goodbye_msg = String::from("Be at peace. (Press enter to exit)");
  println!("{}", goodbye_msg.purple());
  r.store(false, Ordering::SeqCst);
  return;
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
    let r = running.clone();

    ctrlc::set_handler(move || exit(&r)).expect("Error setting ctrlc handler");

    let prompt = String::from("Hello! What's troublin' yer noggin'?");
    println!("{}", prompt.purple());

    while running.load(Ordering::SeqCst) {
      let mut message = String::new();
      let _ = stdout().flush();
      stdin().read_line(&mut message).expect(&prompt);
      clearscreen();
    }
    return;
}
