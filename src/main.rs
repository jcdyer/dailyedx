#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rocket_contrib;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::State;
use rocket_contrib::JSON;

use self::assignments::Assignment;

mod assignments;
mod keys;
mod data;

struct AssignmentState(Arc<Mutex<HashMap<String, Assignment>>>);

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn someone(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/count/<count>")]
fn how_many(count: Result<u8, &str>) -> String {
    match count {
        Ok(n) => format!("Counted {}", n),
        Err(err) => format!("Couldn't count {}", err),
    }
}

#[get("/assignment/<learner>")] 
fn assignment(learner: String, asgn_state: State<AssignmentState>) -> JSON<Assignment> {
    let mut asgns = asgn_state.0.lock().unwrap();
    let asgn = asgns.entry(learner.clone()).or_insert_with(|| {
        println!("Creating new entry for {}", learner);
        let mut blocks = vec![];
        for course in data::get_courses(&learner) {
            for block in data::get_blocks(&course) {
                blocks.push(block);
            }
        }
        Assignment::new(learner, blocks)
    });
    JSON(asgn.clone())
}

#[post("/block/<learner>/<key>")]
fn complete_block(learner: String, key: Option<keys::UsageKey>, asgn_state: State<AssignmentState>) -> Result<JSON<Assignment>, ()> {
    let mut asgns = asgn_state.0.lock().unwrap();
    if let Some(asgn) = asgns.get_mut(&learner) {
        match key {
            Some(usage_key) => {
                if let Some(_) = asgn.units.iter().find(|x| x == &&usage_key) {
                    asgn.increment_completed();
                    Ok(JSON(asgn.clone()))
                } else {
                    Err(())
                }
            },
            None => Err(()),
        }
    } else {
        Err(())
    }
}

fn main() {
    let mut asgns: AssignmentState = AssignmentState(Arc::new(Mutex::new(HashMap::new())));
    rocket::ignite().mount(
        "/", 
        routes![
            world,
            someone,
            how_many,
            assignment,
            complete_block,
        ])
        .manage(asgns)
        .launch();
}
