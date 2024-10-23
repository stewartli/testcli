use std::path::Path;

pub struct Config{
    target: String, 
    name: String,
}

impl Config{
    pub fn look(&self){
        if check_exist(&self.target){
            println!("found it");
        }else{
            println!("nothing there");
        }
    }
    pub fn build(mut input: impl Iterator<Item = String>) -> Self{
        input.next();
        let target = if let Some(t) = input.next(){
            t
        }else{
            "".to_owned()
        };
        let name = if let Some(n) = input.next(){
            n
        }else{
            "".to_owned()
        };
        Self{target, name}
    }
}

pub fn run(config: Config){
    let Config{target: tp, name: nx} = config; 
    println!("the operation [{nx}] is running from the location [{tp}]");
}

fn check_exist<T: AsRef<Path>>(target: T) -> bool{
    target.as_ref().exists()
}
