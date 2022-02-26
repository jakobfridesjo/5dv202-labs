/**
 * Description: Defines structs for forms (HTTP POST) interpretation
 * and for Template context (Tera template engine)
 * 
 * Author: Jakob Fridesjö
 * Date: 2022-02-11
 */

use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct IndexContext<String> {
    pub bar: String,
}

#[derive(Serialize, Debug)]
pub struct MediaContext {
    pub medias: Vec<Media>
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Media {
    pub media_name: String,
    pub media_genre: String,
    pub media_year: i32,
    pub media_score: i32,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Actor {
    pub actor_first_name: String,
    pub actor_last_name: String,
    pub actor_year: i32,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Roles {
    pub roles_role: String,
    pub actor_last_name: String,
    pub role_id: i32,
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct MediaName {
    pub media_name: String,
}