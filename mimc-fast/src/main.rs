use darkforest::{mimc_hash, threshold, ChunkFootprint, Coords, Planet};
use itertools::iproduct;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use warp::{http::Method, Filter};

async fn mine(task: Task) -> Result<impl warp::Reply, warp::Rejection> {
    let x = task.chunkFootprint.bottomLeft.x;
    let y = task.chunkFootprint.bottomLeft.y;
    let size = task.chunkFootprint.sideLength;
    let key = task.planetHashKey;

    let threshold = threshold(task.planetRarity);

    let planets = iproduct!(x..(x + size), y..(y + size))
        .par_bridge()
        .filter_map(|(xi, yi)| {
            let hash = mimc_hash(xi, yi, key);
            if hash < threshold {
                Some(Planet {
                    coords: Coords { x: xi, y: yi },
                    hash: hash.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<Planet>>();

    let rsp = Response {
        chunkFootprint: task.chunkFootprint,
        planetLocations: planets,
    };

    Ok(warp::reply::json(&rsp))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let log = warp::log("mimc-fast");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse::<u16>()
        .unwrap();

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::OPTIONS])
        .allow_any_origin()
        .allow_header("content-type");

    let route = warp::post()
        .and(warp::path("mine"))
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(mine)
        .with(log)
        .with(cors);

    warp::serve(route).run(([0, 0, 0, 0], port)).await
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Task {
    pub chunkFootprint: ChunkFootprint,
    pub planetRarity: u32,
    pub planetHashKey: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct Response {
    pub chunkFootprint: ChunkFootprint,
    pub planetLocations: Vec<Planet>,
}
