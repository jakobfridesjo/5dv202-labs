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
        println!("Error deleting media");
    }
    Redirect::to(uri!(medias))
}

/**
 * Adds a media
 */
#[post("/medias/add", data = "<media>")]
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
 * Deletes a media
 */
#[post("/actors/delete", data = "<actor_name>")]
async fn delete_actor(conn: PsqlConn, actor_name: Form<ActorName>) -> Redirect {
    let a_name: ActorName = actor_name.into_inner();
    let result = conn.run(|c| db_delete_actor(c, a_name)).await;
    if result.is_err() {
        println!("Error deleting actor");
    }
    Redirect::to(uri!(medias))
}

/**
 * Adds an actor
 */
#[post("/actors/add", data = "<actor>")]
async fn add_actor(conn: PsqlConn, actor: Form<Actor>) -> Redirect {
    let a: Actor = actor.into_inner();
    let result = conn.run(|c| db_insert_actor(c, a)).await;
    if result.is_err() {
        println!("Error adding actor");
    }
    Redirect::to(uri!(actors))
}

/**
 * Renders the actors page
 */
#[get("/actors")]
async fn actors(conn: PsqlConn) -> Template {
    let actors_vec: Vec<Actor> = conn.run(|c| db_load_actors(c)).await.unwrap();
    let context = ActorsContext {actors: actors_vec};
    Template::render("actors", &context)
}

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
        .mount("/", routes![index,medias,delete_media,add_media,actors,add_actor, delete_actor])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}