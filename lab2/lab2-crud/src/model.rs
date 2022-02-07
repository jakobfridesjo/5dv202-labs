use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Snack {
    pub name: String,
    pub amount: i32,
    pub price: i32,
}