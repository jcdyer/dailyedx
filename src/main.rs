#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate rocket_contrib;

use rocket_contrib::JSON;

mod assignments;
mod keys;
mod data;

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
fn assignment(learner: String) -> JSON<assignments::Assignment> {
    let mut blocks = vec![];
    for course in data::get_courses(&learner) {
        for block in data::get_blocks(&course) {
            blocks.push(block);
        }
    }
    JSON(assignments::Assignment::new(learner, blocks))
}

fn main() {
    rocket::ignite().mount("/", routes![world, someone, how_many, assignment]).launch();
}
