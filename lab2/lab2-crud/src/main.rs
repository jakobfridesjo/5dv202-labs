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

fn load_from_db(conn: &postgres::Client) -> Vec<Media> {
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
    let medias = Vec::from([media0,media1,media2]);    // Do something with connection, return some data.
    return medias
}

#[get("/medias")]
async fn my_handler(conn: PsqlConn) -> Template {
    conn.run(|c| load_from_db(c)).await;

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
    
        /* Get information from database */
        //let medias: Vec<Media> = db_get_all_medias().unwrap();
        
        let context = MediaContext { medias: vec![media0,media1,media2] };
        Template::render("medias", &context)
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

    /* Get information from database */
    //let medias: Vec<Media> = db_get_all_medias().unwrap();
    
    let context = MediaContext { medias: vec![media0,media1,media2] };
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


    /* Launch rocket! */
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index,medias])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}