use std::thread;

fn main() {
  let threads = (0..25).iter().map(|_| thread::spawn(|| {
    println!("Hello from thread");
    //I forget how to sleep in rust
  }).collect::<Vec<_>>();
  for thread in threads { thread.join(); }
}