
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









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
