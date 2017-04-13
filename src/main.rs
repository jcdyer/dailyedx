#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rocket_contrib;

use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::JSON;

use self::assignments::Assignment;
use self::date::Date;

mod assignments;
mod data;
mod date;
mod keys;

struct AssignmentState(Arc<Mutex<HashMap<(String, Date), Assignment>>>);

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/index.html")
}

#[get("/frontend/build/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("frontend/build/").join(file))
}

#[get("/<learner>/<dt>")]
fn assignment(learner: String, dt: Date, asgn_state: State<AssignmentState>) -> Option<JSON<Assignment>> {
    let mut asgns = asgn_state.0.lock().unwrap();
    let asgn = asgns.entry((learner.clone(), dt.clone())).or_insert_with(move || {
        println!("Creating new entry for {} on {}", learner, dt.clone());
        let mut blocks = vec![];
        for course in data::get_courses(&learner) {
            for block in data::get_blocks(&course) {
                blocks.push(block);
            }
        }
        Assignment::new(learner, dt, blocks)
    });
    Some(JSON(asgn.clone()))
}


#[post("/<learner>/<dt>")]
fn complete_block(learner: String, dt: Date, asgn_state: State<AssignmentState>) -> Option<JSON<Assignment>> {
    let mut asgns = asgn_state.0.lock().unwrap();
    if let Some(asgn) = asgns.get_mut(&(learner.clone(), dt)) {
        asgn.increment_completed();
        Some(JSON(asgn.clone()))
    } else {
        None
    }
}

#[delete("/<learner>/<dt>")]
fn uncomplete_block(learner: String, dt: Date, asgn_state: State<AssignmentState>) -> Option<JSON<Assignment>> {
    let mut asgns = asgn_state.0.lock().unwrap();
    if let Some(asgn) = asgns.get_mut(&(learner.clone(), dt)) {
        asgn.decrement_completed();
        Some(JSON(asgn.clone()))
    } else {
        None
    }
}

fn main() {
    let asgns: AssignmentState = AssignmentState(Arc::new(Mutex::new(HashMap::new())));
    rocket::ignite().mount(
        "/dailyedx",
        routes![
            index,
            files,
            assignment,
            complete_block,
            uncomplete_block,
        ])
        .manage(asgns)
        .launch();
}
