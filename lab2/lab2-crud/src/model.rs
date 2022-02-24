use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct IndexContext<String> {
    pub bar: String,
}

#[derive(Serialize)]
pub struct MediaContext {
    pub medias: Vec<Media>
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub name: String,
    pub genre: String,
    pub year: i32,
    pub score: i32,
}