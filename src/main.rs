#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{JSON, Value};

#[derive(FromForm)]
struct IndexQuery {
    id: String
}

#[get("/?<query>")]
fn index(query: IndexQuery) -> JSON<Value> {
    return JSON(json!({
        "id": query.id,
        "data": {
            "el_texto": "a la grande le puse cuca",
        },
        "params": {
            "param1": 345,
        },
    }));
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
