use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct IndexContext<'a> {
    pub bar: &'a str,
}

#[derive(Serialize)]
pub struct SnacksContext {
    pub snacks: Vec<Snack>
}

#[derive(Serialize, Deserialize)]
pub struct Snack {
    pub name: &'static str,
    pub amount: i32,
    pub price: i32,
}