use std::env;
use std::fs::File;

use std::str::FromStr;

use serde_json::{self, Value}; 

use super::date::Date;
use super::keys::{CourseKey, UsageKey};


pub fn get_courses(learner: &str) -> Vec<CourseKey> {
    let js = json_file("enrollments.json").unwrap();
    if let Value::Array(ref enrollments) = js[learner] {
        parseable(
            enrollments.iter()
                .filter_map(|el| el.as_str())
                .collect()
        )
    } else {
        vec![]
    }
}
        

pub fn get_blocks(course: &CourseKey, date: Date) -> Vec<UsageKey> {
    let js = json_file("blocks.json").unwrap();
    if let Value::Array(ref arr) = js[format!("{}", course)][format!("{}", date)] {
        parseable(
            arr.iter()
                .filter_map(|el| el.as_str())
                .collect()
        )
    } else {
        vec![]
    }
}


fn parseable<T: FromStr>(entries: Vec<&str>) -> Vec<T> {
    entries.iter()
        .map(|x| x.parse::<T>())
        .filter_map(|x| x.ok())
        .collect()
}


fn json_file(name: &str) -> serde_json::Result<Value> {
    let mut path = env::current_dir().unwrap();
    path.push("static");
    path.push(name);
    let f = File::open(path.as_path())?;
    serde_json::from_reader(f)
}
