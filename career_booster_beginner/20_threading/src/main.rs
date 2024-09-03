// Rust provides threading capabilities and asynchronous communication primitives in the standard library. Rust also helps minimize mistakes through the `Send` and `Sync` traits, which indicate whether instances of a type can be sent to another thread and shared between multiple threads, respectively.

// Threading
fn threading() {
  // The `std::thread::spawn` function creates a new thread and runs the closure passed to it.
  // The `std::thread::yield_now` function is used to yield the current thread's execution to another thread.
  // The `handle.join().unwrap()` function waits for the spawned thread to finish executing.
  let handle = std::thread::spawn(|| {
    for i in 0..100 {
      println!("spawned: {i}");
      std::thread::yield_now();
    }
  });
  for i in 0..100 {
    println!("main: {i}");
    std::thread::yield_now();
  }
  handle.join().unwrap();
}

/*
The above may print something like
main: 0
main: 1
...
main: 28
main: 29
spawned: 0
main: 30
spawned: 1
main: 31
spawned: 2
...
 */

// Synchronization
// One method to communicate between threads is through so-called channels.
fn channels() {
  // Create a simple streaming channel.
  let (tx1, rx) = std::sync::mpsc::channel();

  // Copy the producer.
  let tx2 = tx1.clone();

  std::thread::spawn(move || {
    tx1.send(1).unwrap();
  });

  std::thread::spawn(move || {
    tx2.send(2).unwrap();
  });

  // Wait until you receive two messages on the main thread.
  println!("{}", rx.recv().unwrap());
  println!("{}", rx.recv().unwrap());
}
/*
The above may print something like
```

```
But also
```
2
1
```
 */
// 📌 You may find many other synchronization primitives available in the std::sync documentation.

fn main() {}
