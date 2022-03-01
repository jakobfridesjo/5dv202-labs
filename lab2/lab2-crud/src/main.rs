#[macro_use] extern crate rocket;

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
#[post("/medias/delete", data = "<media_form>")]
async fn delete_media(conn: PsqlConn, media_form: Form<MediaForm>) -> Redirect {
    let m_name: MediaForm = media_form.into_inner();
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
#[post("/actors/delete", data = "<actor_form>")]
async fn delete_actor(conn: PsqlConn, actor_form: Form<ActorForm>) -> Redirect {
    let a_name: ActorForm = actor_form.into_inner();
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
 * Deletes a media
 */
#[post("/roles/delete", data = "<role_form>")]
async fn delete_role(conn: PsqlConn, role_form: Form<RoleDeleteForm>) -> Redirect {
    let r_form: RoleDeleteForm = role_form.into_inner();
    let result = conn.run(|c| db_delete_role(c,r_form)).await;
    if result.is_err() {
        println!("Error deleting role");
    }
    Redirect::to(uri!(roles))
}

/**
 * Adds an actor
 */
#[post("/roles/add", data = "<actor>")]
async fn add_role(conn: PsqlConn, actor: Form<RoleAddForm>) -> Redirect {
    let r: RoleAddForm = actor.into_inner();
    let result = conn.run(|c| db_insert_role(c,r)).await;
    if result.is_err() {
        println!("Error adding role");
    }
    Redirect::to(uri!(roles))
}

/**
 * Renders the actors page
 */
#[get("/roles")]
async fn roles(conn: PsqlConn) -> Template {
    let roles_vec: Vec<RoleAddForm> = conn.run(|c| db_load_roles(c)).await.unwrap();
    let context = RolesContext {roles: roles_vec};
    Template::render("roles", &context)
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
        .mount("/", routes![index,medias,delete_media,add_media,actors,add_actor, delete_actor, roles, add_role, delete_role])
        .attach(Template::fairing())
        .attach(PsqlConn::fairing())
}