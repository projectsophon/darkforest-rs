use darkforest::{mimc, threshold, ChunkFootprint, Coords, Planet};
use http_types::headers::HeaderValue;
use itertools::iproduct;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tide::security::{CorsMiddleware, Origin};
use tide::{Body, Request};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::new();
    app.with(cors);

    app.at("/mine").post(|mut req: Request<()>| async move {
        #[allow(non_snake_case)]
        let Task {
            chunkFootprint,
            planetHashKey,
            planetRarity,
        } = req.body_json().await?;

        let x = chunkFootprint.bottomLeft.x;
        let y = chunkFootprint.bottomLeft.y;
        let size = chunkFootprint.sideLength;
        let key = planetHashKey;

        let threshold = threshold(planetRarity);

        let planets = iproduct!(x..(x + size), y..(y + size))
            .par_bridge()
            .filter_map(|(xi, yi)| {
                let hash = mimc(xi, yi, 220, key);
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
            chunkFootprint,
            planetLocations: planets,
        };

        Body::from_json(&rsp)
    });

    app.listen("127.0.0.1:8000").await?;

    Ok(())
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
