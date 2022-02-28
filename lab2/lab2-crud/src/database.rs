/**
 * Description: To be used for communicating with postgresql
 * 
 * Author: Jakob Fridesjö
 * Date: 2022-02-11
 */

use crate::model::*;
use rocket_sync_db_pools::{database, postgres};
use postgres::error::Error;

#[database("psql_pool")]
pub struct PsqlConn(postgres::Client);

/**
 * Loads all medias from database
 */
pub fn db_load_medias(conn: &mut postgres::Client) -> Result<Vec<Media>, Error> {
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
 * Loads all medias from database
 */
pub fn db_load_actors(conn: &mut postgres::Client) -> Result<Vec<Actor>, Error> {
    let mut actors : Vec<Actor> = Vec::new();
    for row in conn.query(
        "SELECT actor_first_name,actor_last_name,actor_year FROM Actor", &[])? {
        actors.push(Actor {
            actor_first_name: row.get(0),
            actor_last_name: row.get(1),
            actor_year: row.get(2),
        });
    }

    Ok(actors)
}

/**
 * Inserts/updates a media in/into database
 */
pub fn db_insert_media(conn: &mut postgres::Client, media: Media) -> Result<(), Error> {

    let mut add: bool = true;
    for _row in conn.query(
        "SELECT * FROM Media WHERE media_name=$1",
        &[&media.media_name]
    )? {
        add = false;
        break;
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
        println!("{}:{}:{}:{}", media.media_name, media.media_genre, media.media_year, media.media_score);
        conn.execute(
            "UPDATE Media
            SET 
                media_name=$1,media_genre=$2,media_year=$3,media_score=$4
            WHERE media_name=$1 
            ",
            &[&media.media_name, &media.media_genre, &media.media_year, &media.media_score]
        )?;
    }

    Ok(())
}

/**
 * Deletes a media from database
 */
pub fn db_delete_media(conn: &mut postgres::Client, media_name: MediaName) -> Result<(), Error> {
    conn.execute(
        "DELETE FROM Media WHERE media_name=$1",
        &[&media_name.media_name] 
    )?;

    Ok(())
}

/**
 * Inserts/updates a media in/into database
 */
pub fn db_insert_actor(conn: &mut postgres::Client, actor: Actor) -> Result<(), Error> {

    let mut add: bool = true;
    for _row in conn.query(
        "SELECT * FROM Actor WHERE actor_first_name=$1 AND actor_last_name=$2",
        &[&actor.actor_first_name,&actor.actor_last_name]
    )? {
        add = false;
        break;
    }
    if add {
        conn.execute(
            "INSERT INTO Actor 
                (actor_first_name,actor_last_name,actor_year) 
            VALUES 
                ($1 ,$2, $3)",
            &[&actor.actor_first_name, &actor.actor_last_name, &actor.actor_year]
        )?;
    }
    else {
        conn.execute(
            "UPDATE Actor
            SET 
                actor_year=$3
            WHERE actor_first_name=$1 AND actor_last_name=$2 
            ",
            &[&actor.actor_first_name, &actor.actor_last_name, &actor.actor_year]
        )?;
    }

    Ok(())
}

/**
 * Deletes a media from database
 */
pub fn db_delete_actor(conn: &mut postgres::Client, actor_name: ActorName) -> Result<(), Error> {
    conn.execute(
        "DELETE FROM Actor WHERE actor_first_name=$1 AND actor_last_name=$2",
        &[&actor_name.actor_first_name,&actor_name.actor_last_name] 
    )?;

    Ok(())
}