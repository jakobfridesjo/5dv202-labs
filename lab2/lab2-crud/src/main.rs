#[macro_use] extern crate rocket;

use model::Media;
use model::MediaContext;
use model::IndexContext;
mod model;
mod database;

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


#[get("/medias")]
fn medias() -> Template {
    let media0 = Media {
        name: "Yakuza 0", 
        genre: "Meme",
        year: 2016,
        score: 100,
    };
    let media1 = Media {
        name: "Yakuza Kiwami", 
        genre: "Meme",
        year: 2017,
        score: 80,
    };
    let media2 = Media {
        name: "Yakuza Kiwami 2", 
        genre: "Meme",
        year: 2018,
        score: 90,
    };

    let context = MediaContext { medias: vec![media0, media1, media2]};
    Template::render("medias", &context)
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
        .mount("/", routes![index,medias])
        .attach(Template::fairing())
}