pub use mimc::U512;
use mimc::{sponge, PrimeElem};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::ops::Div;

mod explorers;
pub use explorers::*;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub fn mimc_hash(x: i64, y: i64, key: u32) -> U512 {
    mimc_hash_rounds(x, y, key, C.len())
}

pub fn mimc_hash_rounds(x: i64, y: i64, key: u32, rounds: usize) -> U512 {
    assert!(rounds <= C.len());
    sponge(&[x, y], 1, rounds, key, &P, &C)[0].x
}

pub fn threshold(rarity: u32) -> U512 {
    P.div(U512::from(rarity))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet {
    pub coords: Coords,
    pub hash: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ChunkFootprint {
    pub bottomLeft: Coords,
    pub sideLength: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_p() {
        let p: U512 = U512::from_dec_str(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
        )
        .unwrap();

        assert_eq!(P, p);
    }

    #[test]
    fn sponge() {
        let hash = mimc_hash(216, 158, 8);
        assert_eq!(
            hash.to_string(),
            String::from(
                "243586509754089793444036766672578053539607441572992882184488791828676079"
            )
        )
    }
}
