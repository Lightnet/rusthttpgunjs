/*
 * Library Name: gunrs
 * 
 * Information:
 *  Gunrs is in work in progress.
 * 
 * Notes:
 * - Need get json and function working together in rust language?
 * - Need to create different logic that follow javascript to rust logic?
 * 
*/

//https://play.rust-lang.org/?gist=4cc003f82a2d53a60a1bae1436c84ad5&version=stable&mode=debug
// https://users.rust-lang.org/t/how-to-implement-a-hashmap-of-vectors-of-any-type/18341
// https://users.rust-lang.org/t/convert-box-dyn-t-to-box-dyn-any/28951/8
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=288a134fecaa31f1576e74d5f0316812
// https://users.rust-lang.org/t/convert-box-dyn-t-to-box-dyn-any/28951
// https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/dyn-trait-for-trait-objects.html
#[allow(dead_code)]
use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;

pub use self::ham::Ham;
mod ham;

pub use self::dup::Dup;
mod dup;

pub use self::store::Store;
mod store;

pub trait Key: Any {
    type Value: Any;
}

pub struct Gun {
    version: String,
    #[allow(dead_code)]
    opt: HashMap<TypeId, Box<dyn Any>>,
    //name: String,
    #[allow(dead_code)]
    path: String,
    store: Store,
    #[allow(dead_code)]
    dup:Dup,
    #[allow(dead_code)]
    ham:Ham
}

impl Gun {
    #[allow(dead_code)]
    pub fn new() -> Gun { //create gun
        Gun{ //init object
           version: "0.1.0".to_string(), //display version
           opt: HashMap::new(), //need on work config
           path: "".to_string(), //just testing
           store: Store::new(), //partly tested
           dup: Dup::new(), //placeholder
           ham: Ham::new() //placeholder
        }// return Gun instance
    }
    //need to learn how get to return value
    #[allow(dead_code)]
    pub fn get(&self, data: &str) {
        println!("get {}", data);
        self.store.get(&"radata".to_string());
    }

    #[allow(dead_code)]
    pub fn put(&self,_data: &str) {
        println!("put {}", _data);
        self.store.put(&"radata".to_string(),_data);
        //println!("put test!");
    }

    //testing below code!
    #[allow(dead_code)]
    pub fn show(&self) {
        println!("version {}", self.version.clone());
    }
    #[allow(dead_code)]
    pub fn store_write(&self) {
        self.store.writefile();
    }
    #[allow(dead_code)]
    pub fn store_read(&self) {
        self.store.readfile();
    }
}

//default generate code
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
