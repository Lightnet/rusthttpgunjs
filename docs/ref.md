




# Links:
 * https://www.rust-lang.org/learn/get-started
 * https://doc.rust-lang.org/book/ch20-01-single-threaded.html
 * https://doc.rust-lang.org/1.0.0/book/installing-rust.html
 * https://forge.rust-lang.org/other-installation-methods.html#standalone
 
 * https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d
 * https://crates.io/crates/actix-web
 * 

 
```rust
fn get_two_sites() {
    // Spawn two threads to do work.
    let thread_one = thread::spawn(|| download("https:://www.foo.com"));
    let thread_two = thread::spawn(|| download("https:://www.bar.com"));

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
```