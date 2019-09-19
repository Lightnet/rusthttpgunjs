/*
 * Library Name: gunrs
 * 
 * Information:
 *  Gunrs is in work in progress.
 * 
 * Notes:
 * - 
 * 
*/


// https://users.rust-lang.org/t/rustify-combinators-application-to-create-map-from-string/4819/2
// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
/*
function Dup(){
    var dup = {s:{}}, opt = {max: 1000, age: 1000 * 9};
    dup.check = function(id){
        return dup.s[id]? dup.track(id) : false;
    }
    dup.track = function(id){
        dup.s[id] = (+new Date());
        if(!dup.to){
            dup.to = setTimeout(function(){
                Object.keys(dup.s).forEach(function(time, id){
                    if(opt.age > ((+new Date()) - time)){ return }
                    delete dup.s[id];
                });
                dup.to = null;
            }, opt.age);
        }
        return id;
    }
    return dup;
}

Dup.random = function(){ return Math.random().toString(36).slice(-3) }
try{module.exports = Dup}catch(e){}
*/

use std::collections::HashMap;
#[allow(dead_code)]
//use std::any::Any;

#[allow(dead_code)]
pub struct Opt{
    max: i32,// = 1000,
    age: i32,// = 1000 * 9
}

#[allow(dead_code)]
pub struct Dup{
    s: HashMap<String,String>,
    opt: Opt
}

impl Dup {
    #[allow(dead_code)]
    pub fn new() -> Dup{
        Dup{
            s:HashMap::new(),
            opt: Opt{
                max:1000,
                age:9000
            }
        }//return Dup
    }

    #[allow(dead_code)]
    pub fn check(){

    }
    #[allow(dead_code)]
    pub fn track(){
        
    }
    #[allow(dead_code)]
    pub fn random(){
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //assert_eq!(2 + 2, 4);
        println!("test dup");
        //let dup = Dup();


    }
}