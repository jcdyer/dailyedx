use std::collections::HashMap;

pub static mut VARS: Option<HashMap<&'static str, &'static str>> = None;


pub fn init_vars() {
    unsafe {
        let mut map = HashMap::new();
        map.insert("you", "world");
        VARS = Some(map);
    }
}

pub fn get_var(key: &str) -> &'static str {
    unsafe {
        VARS.unwrap().get(key).unwrap().clone()
    }
}

fn main() {
    init_vars();
    let x = get_var("you");
    println!("Hello {}.", x);
}


