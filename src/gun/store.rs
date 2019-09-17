



#[derive(Debug)]
pub struct Store(i8);

#[allow(dead_code)]
impl Store {
    pub fn new() -> Store { 
        println!("New Gun?");
        Store(4) 
    }
}