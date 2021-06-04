use darkforest::{mimc_hash, threshold, ChunkFootprint, Coords, Planet, SpiralExplorer};
use itertools::iproduct;
use rayon::prelude::*;

// SnarkConstants.PLANETHASH_KEY
static PLANETHASH_KEY: u32 = 420;
// GameConstants.PLANET_RARITY
static PLANET_RARITY: u32 = 16384;

fn main() {
    let center = Coords { x: 0, y: 0 };
    let chunk_side_length = 256;
    let mut explorer = SpiralExplorer::new(center, chunk_side_length);

    loop {
        if let Some(chunk) = explorer.next() {
            let found = mine(chunk.clone());
            // store in database or something?
            println!("{:?} {:?} ", chunk, found);
        }
    }
}

fn mine(chunk: ChunkFootprint) -> Vec<Planet> {
    let ChunkFootprint {
        sideLength: size,
        bottomLeft: Coords { x, y },
    } = chunk;

    let threshold = threshold(PLANET_RARITY);

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
