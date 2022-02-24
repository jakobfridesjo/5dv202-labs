#[macro_use] extern crate rocket;

use model::Media;
use model::MediaContext;
use model::IndexContext;
mod model;

use crate::database::*;
mod database;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use serde::Serialize;
use rocket_sync_db_pools::{database, postgres};

#[database("psql_pool")]
struct PsqlConn(postgres::Client);

/*#[get("/logs/<id>")]
async fn get_logs(conn: LogsDbConn, id: usize) -> Result<Logs> {
    conn.run(|c| Logs::by_id(c, id)).await
}*/

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
    /* Insert some data into database */
    let media0 = Media {
        name: "Yakuza 0".to_string(), 
        genre: "Meme".to_string(),
        year: 2016,
        score: 100,
    };
    let media1 = Media {
        name: "Yakuza Kiwami".to_string(), 
        genre: "Meme".to_string(),
        year: 2017,
        score: 80,
    };
    let media2 = Media {
        name: "Yakuza Kiwami 2".to_string(), 
        genre: "Meme".to_string(),
        year: 2018,
        score: 90,
    };
    
    let context = MediaContext { medias: vec![media0, media1, media2]};
    Template::render("medias", &context)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", IndexContext {
        bar: "Hello World!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    
    /* Create database */
    /*let mut client = Client::connect("host=localhost user=postgres", NoTls);

    /* Insert some data into database */
    let media0 = Media {
        name: "Yakuza 0".to_string(), 
        genre: "Meme".to_string(),
        year: 2016,
        score: 100,
    };
    let media1 = Media {
        name: "Yakuza Kiwami".to_string(), 
        genre: "Meme".to_string(),
        year: 2017,
        score: 80,
    };
    let media2 = Media {
        name: "Yakuza Kiwami 2".to_string(), 
        genre: "Meme".to_string(),
        year: 2018,
        score: 90,
    };
    */


    /* Launch rocket! */
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index,medias])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}