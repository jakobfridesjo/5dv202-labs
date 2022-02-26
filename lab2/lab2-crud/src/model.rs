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