use std::thread;

fn main() {
  use std::{thread, time};

  let timething = time::Duration::from_millis(30000);

  let threads = (0..256).map(|_| thread::spawn(move || {
    thread::sleep(timething);
    println!("Hello from thread");
  })).collect::<Vec<_>>();
  for thread in threads { thread.join(); }
}