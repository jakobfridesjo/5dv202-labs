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
use postgres::error::Error;

#[database("psql_pool")]
struct PsqlConn(postgres::Client);

/**
 * Loads all medias from database
 */
fn db_load_medias(conn: &mut postgres::Client) -> Result<Vec<Media>, Error> {
    let mut medias : Vec<Media> = Vec::new();
    for row in conn.query(
        "SELECT media_name,media_genre,media_year,media_score FROM Media;", &[])? {
        medias.push(Media {
            name: row.get(0),
            genre: row.get(1),
            year: row.get(2),
            score: row.get(3),
        });
    }
    Ok(medias)
}

/**
 * Inserts a media into database
 */
fn db_insert_medias(conn: &mut postgres::Client, medias: Vec<Media>) -> Result<(), Error> {

    for media in medias {
        println!("{}:{}:{}:{}", media.score, media.genre, media.year, media.score);
        conn.execute(
            "INSERT INTO Media (media_name,media_genre,media_year,media_score) 
            VALUES ($1 ,$2, $3, $4);",
            &[&media.score, &media.genre, &media.year, &media.score]
        )?;
    }

    Ok(())
}

/**
 * Deletes a media from database
 */
fn db_delete_media(conn: &mut postgres::Client, media_name: String) -> Result<(), Error> {
    println!("{}", media_name);
    conn.execute(
        "DELETE FROM Media WHERE media_name=$1;",
        &[&media_name] 
    )?;

    Ok(())
}

/**
 * Deletes a media
 */
#[post("/medias/delete", data = "<media_name>")]
async fn delete_media(conn: PsqlConn, media_name: String) {
    conn.run(|c| db_delete_media(c, media_name)).await;
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
        .mount("/", FileServer::from("static"))
        .mount("/", routes![index,medias,delete_media])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}