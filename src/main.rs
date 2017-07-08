#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;

mod bots;

use rocket_contrib::{JSON, Value};
use std::collections::HashMap;

use bots::Bot;

#[get("/")]
fn index() -> JSON<Value> {
    return JSON(json!({ "ok": true}));
}

#[get("/<name>")]
fn botname(name: &str) -> JSON<Value> {
    return JSON(json!({
        "name": name,
    }));
}

fn main() {
    let mut bots:HashMap<&str, Bot> = HashMap::new();

    rocket::ignite()
        .mount("/", routes![index, botname])
        .launch();
}
