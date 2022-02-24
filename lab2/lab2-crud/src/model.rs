use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct IndexContext<String> {
    pub bar: String,
}

#[derive(Serialize, Debug)]
pub struct MediaContext {
    pub medias: Vec<Media>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub name: String,
    pub genre: String,
    pub year: i32,
    pub score: i32,
}