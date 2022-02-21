use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct IndexContext<'a> {
    pub bar: &'a str,
}

#[derive(Serialize)]
pub struct MediaContext {
    pub medias: Vec<Media>
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub name: &'static str,
    pub genre: &'static str,
    pub year: i32,
    pub score: i32,
}