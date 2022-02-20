#[macro_use] extern crate rocket;

use model::Snack;
use model::SnacksContext;
use model::IndexContext;
mod model;
//mod database;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use serde::Serialize;

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
    let snack0 = Snack {
        name: "ananas", 
        amount: 1, 
        price: 20,
    };
    let snack1 = Snack {
        name: "mango", 
        amount: 2, 
        price: 30,
    };
    let snack2 = Snack {
        name: "apelsin", 
        amount: 10, 
        price: 20,
    };

    let context = SnacksContext { snacks: vec![snack0, snack1, snack2]};
    Template::render("snacks", &context)
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