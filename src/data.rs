use std::env;
use std::fs::File;

use std::str::FromStr;

use serde_json::{self, Value}; 

use super::date::Date;
use super::keys::{CourseKey, UsageKey};


pub fn get_courses(learner: &str) -> Vec<CourseKey> {
    let json_str = include_str!("../static/enrollments.json");
    let js: Value = serde_json::from_str(json_str).unwrap();
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
    let json_str = include_str!("../static/blocks.json");
    let js: Value = serde_json::from_str(json_str).unwrap();
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
