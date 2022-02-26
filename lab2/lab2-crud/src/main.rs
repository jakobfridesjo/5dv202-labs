#[macro_use] extern crate rocket;

use std::result;

use model::*;
mod model;
use database::*;
mod database;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket::{form::Form, response::Redirect};

/**
 * Deletes a media
 */
#[post("/medias/delete", data = "<media_name>")]
async fn delete_media(conn: PsqlConn, media_name: Form<MediaName>) -> Redirect {
    let m_name: MediaName = media_name.into_inner();
    let result = conn.run(|c| db_delete_media(c, m_name)).await;
    if result.is_err() {
        println!("Error adding media");
    }
    Redirect::to(uri!(medias))
}

/**
 * Deletes a media
 */
#[post("/media/add", data = "<media>")]
async fn add_media(conn: PsqlConn, media: Form<Media>) -> Redirect {
    let m: Media = media.into_inner();
    let result = conn.run(|c| db_insert_media(c, m)).await;
    if result.is_err() {
        println!("Error adding media");
    }
    Redirect::to(uri!(medias))
}

/**
 * Renders the medias page
 */
#[get("/medias")]
async fn medias(conn: PsqlConn) -> Template {
    let medias_vec: Vec<Media> = conn.run(|c| db_load_medias(c)).await.unwrap();
    let context = MediaContext {medias: medias_vec};
    Template::render("medias", &context)
}

/**
 * Renders the actors page
 */
/*#[get("/actors")]
async fn actors(conn: PsqlConn) -> Template {
    //let medias_vec: Vec<Media> = conn.run(|c| db_load_medias(c)).await.unwrap();
    //let context = ActorContext {medias: medias_vec};
    //Template::render("medias", &context)
}*/

/**
 * Renders the index page
 */
#[get("/")]
fn index() -> Template {
    Template::render("index", IndexContext {
        bar: "Hello World!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    
    /* Launch rocket! */
    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index,medias,delete_media,add_media])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}