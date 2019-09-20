


 * https://cheats.rs/
 * https://cheats.rs/#functions-behavior

# Links:
 * https://www.rust-lang.org/learn/get-started
 * https://doc.rust-lang.org/book/ch20-01-single-threaded.html
 * https://doc.rust-lang.org/1.0.0/book/installing-rust.html
 * https://forge.rust-lang.org/other-installation-methods.html#standalone
 * https://gist.github.com/mjohnsullivan/e5182707caf0a9dbdf2d
 * https://crates.io/crates/actix-web
 * https://substrate.dev/rustdocs/v1.0/tokio/index.html
 * https://tokio-rs.github.io/tokio/doc/tokio/fs/struct.File.html
 * https://hyper.rs/guides/server/echo/
 * https://github.com/hyperium/hyper/blob/0.12.x/examples/upgrades.rs
 * https://docs.rs/crate/websocket/0.23.0/source/examples/hyper.rs
 * https://doc.rust-lang.org/std/macro.include_str.html
 * https://docs.rs/crate/websocket/0.23.0/source/examples/hyper.rs
 * https://github.com/actix/examples/blob/0.7/websocket/src/main.rs
 * https://github.com/actix/examples/tree/0.7/websocket
 * https://doc.rust-lang.org/std/keyword.impl.html
 * https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html
 * https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
 * https://rust-lang-nursery.github.io/cli-wg/tutorial/packaging.html
 * https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
 * https://github.com/cyderize/rust-websocket
 * https://nbaksalyar.github.io/2015/07/10/writing-chat-in-rust.html#handshake
 * https://github.com/flosse/rust-web-framework-comparison#websocket-libraries
 * https://crates.parity.io/websocket/async/struct.TcpListener.html
 * https://github.com/tiny-http/tiny-http
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
 * 
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