use std::io;
use std::sync::{Arc, atomic::AtomicBool};
use ctrlc;

fn main() {
  let live = Arc::new(AtomicBool::new(true));
  let live2 = live.clone();

  ctrlc::set_handler(move || {
    println!("Got Ctrl+C. Shutting down...");
    live2.store(false, std::sync::atomic::Ordering::SeqCst);
  }).expect("Failed to set Ctrl+C handler.");

  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(n) => {
        println!("{} bytes read", n);
        println!("{}", input);
        if n == 0 { break; } // Ctrl+C
      }
      Err(error) => println!("error: {}", error),
    }

    if !live.load(std::sync::atomic::Ordering::SeqCst) { break; }
  }
}
