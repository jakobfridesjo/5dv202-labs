mod model;

#[macro_use] extern crate rocket;
//use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct IndexContext<'a> {
    bar: &'a str,
}

#[derive(Serialize)]
struct SnacksContext<'a> {
    bar: &'a str,
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
    Template::render("snacks", SnacksContext {
        bar: "Hello World!",
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
        .mount("/", routes![index,snacks])
        .attach(Template::fairing())
}
