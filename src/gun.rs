//test
//https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html
//https://doc.rust-lang.org/book/ch17-02-trait-objects.html
//https://doc.rust-lang.org/rust-by-example/fn/methods.html
//https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
//

#![warn(dead_code)]
pub use self::store::Store;
mod store;

trait Show {
    fn show(&self) -> String;
}

pub struct Gun {
    version: String,
    //name: String,
    path: String
}

impl Gun {
    #[allow(dead_code)]
    pub fn new() -> Gun {
        Gun{
           version: "0.1.0".to_string(),
           path: "".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn get(&self) {
        println!("get test!");
    }
    #[allow(dead_code)]
    pub fn put(&self) {
        println!("put test!");
    }
    #[allow(dead_code)]
    pub fn show(&self) {
        println!("version {}", self.version.clone());
    }
}

trait Location {
    fn do_location(&self);
}

impl Location for Gun {
    fn do_location(&self){
        println!("path test!");
        //self.path.clone()
    }
}

//trait Put {
    //fn put(&self) -> String;
//}

//impl Put for Gun {
    //fn put(&self) {
        //self.name.clone()
        //println!("put test!");
        //"put".to_string()
    //}
//}

//#![warn(dead_code)] //ingore unused code ???

//#[allow(dead_code)]
//pub fn print_gun() {
    //println!("test gun");
//}

//pub fn get() {
    //println!("test get");
//}

//pub fn put() {
    //println!("test put");
//}

//pub trait Put {
    //fn put(&self);
//}

//#[derive(Debug)]
//pub struct Gun {
    //opt: String,
    //graph: String,
//}

//impl Get for Gun {
    //fn get(&self) {
        //self.name.clone()
    //}
//}

//impl Put for Gun {
    //fn put(&self) {
        //self.name.clone()
    //}
//}

//impl Gun {
    //fn new() -> Gun {
        //Gun{
            //opt: "".to_string(),
            //graph: "".to_string(),
        //}
    //}

    //fn new(name: &str, location: &str) -> Gun {
    //fn new(name: &str, location: &str) -> Gun {
        //Gun{
            //name: name.to_string(),
            //location: location.to_string()
        //}
    //}
//}

