#[macro_use] extern crate rocket;

use model::Media;
use model::MediaContext;
use model::IndexContext;
mod model;

use rocket_dyn_templates::Template;
use rocket::fs::FileServer;
use serde::Serialize;
use rocket_sync_db_pools::{database, postgres};
use postgres::error::Error;
use rocket::form::Form;

#[database("psql_pool")]
struct PsqlConn(postgres::Client);

/**
 * Loads all medias from database
 */
fn db_load_medias(conn: &mut postgres::Client) -> Result<Vec<Media>, Error> {
    let mut medias : Vec<Media> = Vec::new();
    for row in conn.query(
        "SELECT media_name,media_genre,media_year,media_score FROM Media", &[])? {
        medias.push(Media {
            media_name: row.get(0),
            media_genre: row.get(1),
            media_year: row.get(2),
            media_score: row.get(3),
        });
    }

    Ok(medias)
}

/**
 * Inserts/updates a media in/into database
 */
fn db_insert_media(conn: &mut postgres::Client, media: Media) -> Result<(), Error> {

    let mut add: bool = true;
    for row in conn.query(
        "SELECT media_name FROM Media WHERE media.media_name=$1",
        &[]
    )? {
        let name: String = row.get(0);
        if media.media_name.eq(&name) {
            add = false 
        }
    }
    
    if add {
        println!("{}:{}:{}:{}", media.media_name, media.media_genre, media.media_year, media.media_score);
        conn.execute(
            "INSERT INTO Media 
                (media_name,media_genre,media_year,media_score) 
            VALUES 
                ($1 ,$2, $3, $4)",
            &[&media.media_name, &media.media_genre, &media.media_year, &media.media_score]
        )?;
    }
    else {
        conn.execute(
            "UPDATE Media
            SET 
                (media_name,media_genre,media_year,media_score) 
            VALUES 
                ($1 ,$2, $3, $4)",
            &[&media.media_name, &media.media_genre, &media.media_year, &media.media_score]
        )?;
    }

    Ok(())
}

/**
 * Deletes a media from database
 */
fn db_delete_media(conn: &mut postgres::Client, media_name: String) -> Result<(), Error> {
    let query_string: String = media_name[11..].to_string();
    println!("NAME TO REMOVE {}", query_string);
    conn.execute(
        "DELETE FROM Media WHERE media_name=$1",
        &[&query_string] 
    )?;

    Ok(())
}

/**
 * Deletes a media
 */
#[post("/medias/delete", data = "<media_name>")]
async fn delete_media(conn: PsqlConn, media_name: String) -> Template {
    conn.run(|c| db_delete_media(c, media_name)).await;
    let medias_vec: Vec<Media> = conn.run(|c| db_load_medias(c)).await.unwrap();
    let context = MediaContext {medias: medias_vec};
    Template::render("medias", &context)
}

/**
 * Deletes a media
 */
#[post("/media/add", data = "<media>")]
async fn add_media(conn: PsqlConn, media: Form<Media>) -> Template {
    let m: Media = media.into_inner();
    conn.run(|c| db_insert_media(c, m)).await;
    let medias_vec: Vec<Media> = conn.run(|c| db_load_medias(c)).await.unwrap();
    let context = MediaContext {medias: medias_vec};
    Template::render("medias", &context)
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
        .mount("/", routes![index,medias,delete_media,add_media])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}