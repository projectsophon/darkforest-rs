//! This is an example of a standalone miner that doesn't require a Dark Forest
//! game to connect and drive it. It hardcodes the key and rarity which need to
//! be updated based on whatever version of Dark Forest you're running or will
//! produce bad data. You would want to stick the results in a database or
//! somethig, but for this example theyre simply debug printed out as json.
//!
//! cargo run --release --example standalone

use darkforest::{mimc_hash, threshold, ChunkFootprint, Coords, Planet, SpiralExplorer, U512};
use itertools::iproduct;
use rayon::prelude::*;

// SnarkConstants.PLANETHASH_KEY
const PLANETHASH_KEY: u32 = 420;
// GameConstants.PLANET_RARITY
const PLANET_RARITY: u32 = 16384;

fn main() {
    // how big of chunks, client default is 16, remote miner default is 256
    const CHUNK_SIZE: u16 = 256;

    // set where you want to mine here
    let center = Coords { x: 0, y: 0 };

    let threshold: U512 = threshold(PLANET_RARITY);
    let mut explorer = SpiralExplorer::new(center, CHUNK_SIZE);

    loop {
        if let Some(chunk) = explorer.next() {
            let found = explore(chunk.clone(), threshold);
            // storing in a database or flatfile is an exercise left up to reader
            println!("{:?} {:?} ", chunk, found);
        }
    }
}

fn explore(chunk: ChunkFootprint, threshold: U512) -> Vec<Planet> {
    let ChunkFootprint {
        sideLength: size,
        bottomLeft: Coords { x, y },
    } = chunk;

    iproduct!(x..(x + size), y..(y + size))
        .par_bridge()
        .filter_map(|(xi, yi)| {
            let hash = mimc_hash(xi, yi, PLANETHASH_KEY);
            if hash < threshold {
                Some(Planet {
                    coords: Coords { x: xi, y: yi },
                    hash: hash.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<Planet>>()
}
