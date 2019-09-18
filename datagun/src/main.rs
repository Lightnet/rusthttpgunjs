//cargo gun -p datagun
#![warn(unused_imports)]
extern crate gunrs;

//use std::io;
//use std::io::prelude::*;
//use std::fs::File;

use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    //let gun = gunrs::Gun::new();
    //gun.get();

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
    
    let path = "foo.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }

    // and more! See the other methods for more details.
    Ok(())

    //writetest();
}
