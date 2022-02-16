#[macro_use] extern crate rocket;

mod model;
//mod database; Not ready yet

use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket::fs::FileServer;
use serde::Serialize;

#[derive(Serialize)]
struct IndexContext<'a> {
    bar: &'a str,
}

#[derive(Serialize)]
struct SnacksContext {
    name: &'static str,
    amount: i32,
    price: i32,
}

#[post("/")]
fn create() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn read() -> &'static str {
    "Hello, world!"
}

#[put("/")]
fn update() -> &'static str {
    "Hello, world!"
}

#[delete("/")]
fn delete() -> &'static str {
    "Hello, world!"
}


#[get("/snacks")]
fn snacks() -> Template {
    Template::render("snacks", context! {
        name: "Hello World!",
        amount: 0,
        price: 0,
    })
}

#[get("/")]
fn index() -> Template {
    Template::render("index", IndexContext {
        bar: "Hello World!",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index,snacks])
        .attach(Template::fairing())
}