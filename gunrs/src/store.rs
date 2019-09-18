use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Store{
    file:String
}

#[allow(dead_code)]
static LOREM_IPSUM: &str =
    "Hello
world
";

#[allow(dead_code)]
impl Store {
    pub fn new() -> Store { 
        println!("Gun store?");
        Store{
            file: "data".to_string(),
        } //return
    }

    #[allow(dead_code)]
    pub fn writefile(&self) { 
        println!("filename: {}", &self.file);
        let path = Path::new(&self.file);
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

    #[allow(dead_code)]
    pub fn readfile(&self){
        println!("filename: {}", &self.file);
        // Create a path to the desired file
        let path = Path::new(&self.file);
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
}