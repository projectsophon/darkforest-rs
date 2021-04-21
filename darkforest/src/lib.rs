pub use mimc::U512;
use mimc::{MimcState, P};
use serde::{Deserialize, Serialize};
use std::ops::Div;

pub fn mimc(x: i64, y: i64, rounds: usize, key: u32) -> U512 {
    MimcState::sponge(&[x, y], 1, rounds, key)[0].x
}

pub fn threshold(rarity: u32) -> U512 {
    P.div(U512::from(rarity))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Planet {
    pub coords: Coords,
    pub hash: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkFootprint {
    pub bottomLeft: Coords,
    pub sideLength: i64,
}
