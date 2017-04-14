use std::env;
use std::fs::File;

use std::str::FromStr;

use serde_json::{self, Value}; 

use super::date::Date;
use super::keys::{CourseKey, UsageKey};


fn parseable<T: FromStr>(entries: Vec<&str>) -> Vec<T> {
    entries.iter()
        .map(|x| x.parse::<T>())
        .filter_map(|x| x.ok())
        .collect()
}


pub fn get_courses(learner: &str) -> Vec<CourseKey> {

    let mut path = env::current_dir().unwrap();
    path.push("static");
    path.push("enrollments.json");
    let js: Value = match File::open(path.as_path()) {
        Ok(r) => serde_json::from_reader(r).expect("Data file was not valid JSON"),
        Err(_) => return vec![],
    };
    let mut vec = Vec::new();
    if let Value::Array(ref enrollments) = js[learner] {
        for element in enrollments {
            match element {
                &Value::String(ref s) => vec.push(s.as_str()),
                _ => return vec![],
            }
        }
    };
    parseable(vec)
}
        

pub fn get_blocks(course: &CourseKey, date: Date) -> Vec<UsageKey> {
    let mut path = env::current_dir().unwrap();
    path.push("static");
    path.push("blocks.json");
    let js: Value = match File::open(path.as_path()) {
        Ok(r) => serde_json::from_reader(r).expect("Data file was not valid JSON"),
        Err(_) => return vec![],
    };
    let mut vec = Vec::new();
    if let Value::Array(ref arr) = js[format!("{}", course)][format!("{}", date)] {
        for element in arr {
            match element {
                &Value::String(ref s) => vec.push(s.as_str()),
                _ => return vec![],
            }
        }
    };
    parseable(vec)
}
