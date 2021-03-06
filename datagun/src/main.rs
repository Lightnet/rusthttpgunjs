//https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html

//cargo gun -p datagun

#![warn(unused_imports)]
#![warn(dead_code)]
extern crate gunrs;
extern crate serde_json;
//use std::ptr::null;
//https://doc.rust-lang.org/std/hash/trait.Hash.html
use std::collections::HashMap;
//use std::hash::{Hash, Hasher};
//use std::hash::Hash::hash;
use serde_json::{Value};
//use serde_json::json;


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
/*
static PUTSTR: &str =
    "Hello
world test
";
*/

static PUTSTR: &str =
    "Hello world put test";
// cargo run -p datagun

struct Key {
}

//#[derive(Debug, Hash)]
struct Gunlist {
    //key: HashMap<String, u32>,
    //key: HashMap<String, u32>,
    key: Vec<Value>,
}

impl Default for Gunlist {
    fn default() -> Gunlist {
        Gunlist {
            //key:Key{}
            //key: HashMap::new()
            key: Vec::new()
        }
    }
}

impl Gunlist {
    //pub fn new() -> Gunlist {
        //Gunlist {
            //key:Key{}
            //key: HashMap::new(),
        //}
    //}
}


//https://stackoverflow.com/questions/56184109/how-to-convert-vec-to-jsonvalue-in-rust
fn main() {
    //writefile();
    //readfile();
    //
    //let mut apu0 = Gunlist::new();
    let mut apu = Gunlist::default();
    //apu.key;

    //
    let gun = gunrs::Gun::new();
    gun.put(PUTSTR); //ok
    //gun.get(PUTSTR);
    //gun.store_write();
    //gun.store_read();
    //let mut my_string = String::from("How's it going? My name is Alias.");

    //println!("length: {}",my_string.len());
    //println!("String is empty? {}",my_string.is_empty());

    //for token in my_string.split_whitespace(){
        //println!("{}",token);
    //}

    //println!("Does the String contain 'Alias'? {}",my_string.contains("Alias"));

    //my_string.push_str("welcome string add");

    //println!("{}",my_string);

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
