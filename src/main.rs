#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{JSON, Value};

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
    rocket::ignite()
        .mount("/", routes![index, botname])
        .launch();
}
