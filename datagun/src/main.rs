//https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html

//cargo gun -p datagun

#![warn(unused_imports)]
#![warn(dead_code)]
extern crate gunrs;


/*
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
fn readfile(){
    // Create a path to the desired file
    let path = Path::new("foo.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}

#[allow(dead_code)]
static LOREM_IPSUM: &str =
    "Hello
world
";

#[allow(dead_code)]
fn writefile(){
    let path = Path::new("foo.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
*/
fn main() {
    //writefile();
    //readfile();
    let gun = gunrs::Gun::new();
    gun.put();
    gun.store_write();
    gun.store_read();
}


//use std::io;
//use std::io::prelude::*;
//use std::fs::File;

//use std::fs::File;
//use std::io::{Write, BufReader, BufRead, Error};

//https://doc.rust-lang.org/std/io/trait.Write.html
//https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html
//not right code
/*
fn writetest() -> std::io::Result<()> {
    let data = b"some bytes";
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
*/

//fn main() -> Result<(), Error> {
    //println!("Hello, world!");
    //let gun = gunrs::Gun::new();
    //gun.get();
    //method #1
    //let mut f = File::open("foo.txt")?;
    //let mut buffer = [0; 10];
    // read up to 10 bytes
    //f.read(&mut buffer)?;
    //let mut buffer = Vec::new();
    // read the whole file
    //f.read_to_end(&mut buffer)?;
    // read into a String, so that you don't need to do the conversion.
    //let mut buffer = String::new();
    //f.read_to_string(&mut buffer)?;
    
    //method #2
    //let path = "foo.txt";
    //let input = File::open(path)?;
    //let buffered = BufReader::new(input);
    //for line in buffered.lines() {
        //println!("{}", line?);
    //}
    // and more! See the other methods for more details.
    //Ok(())
    //writetest();
//}
