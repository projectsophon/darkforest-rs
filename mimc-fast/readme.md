# Dark forest mimc miner

[Dark Forest](https://zkga.me/), the world's first decentralized real-time strategy game. Built on Ethereum with zkSNARKsis a <https://zkga.me/>

Note if you're not technical and cant get through this guide, remember you can get a lot of speedup by just having the stock game use more cores with something like `df.setMinerCores(16)` in the console. So try that first.

This rust package pulls out the fog of war miner to its own binary with a webserver for in game plugins to talk to it. It provides faster results than the built in javascript miner on the same machine. If you're using it on the same machine as the game Pause your in game miner as theyll just compete with eachother and give WORSE results. It also can run remotely in the cloud or even a rapsberry pi 4 has been found to provide 1-2k hashes.

## Install

The rust miner on the same machine as your game can be faster than the javascript because it can more fully utilize the processor. But if you're running on the same machine as the game, pause the in game miner. They're just going to compete with eachother and give you worse peformance.

- Install [rust for your operating system](https://www.rust-lang.org/tools/install) probably with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install with `cargo install --git  https://github.com/projectsophon/darkforest-rs --bin mimc-fast --force --branch=main`
- Run it (with logging enabled environment variable) with `mimc-fast` or with logging enabled `RUST_LOG=info mimc-fast`
- Connect to it with the built in plugin "Remote Explore"
- Finally if your server does't have an ssl cert on it (it probably doesn't) then you also need to cripple ssl security for the Dark Forest game domain. NOTE THIS IS VERY DANGEROUS. See [enabling mixed content](https://github.com/darkforest-eth/plugins/blob/dc1e024ac658ef34873f8b36176cca2945e52e7c/content/productivity/remote-explore/enable-mixed.md) for an example of how thats done. 

Remember to pause the built in miner if you're running it on the same machine as the game client or they'll just fight eachother for resources and give LESS hashes overall.

## Deploy to Google Cloud Run

- Add a `Dockerfile`:
```docker
FROM rust
WORKDIR /darkforest-rs

COPY . .
RUN cargo build --release --bin mimc-fast

EXPOSE 8000
CMD ["./target/release/mimc-fast"]
```
- Build `docker build -t mimc-fast .`
- Push `docker tag mimc-fast gcr.io/$GCLOUD_PROJECT_ID/mimc-fast && docker push gcr.io/$GCLOUD_PROJECT_ID/mimc-fast`
- Set up a Cloud Run service based on the image you just push. Don't forget to edit the service to set the port to 8000.
- You can now use the URL provided by the cloud run service to call it `/mine` using the remote explorer plugin in-game.

## Troubleshooting
To test its working to the mining url, in the case of your local machine `curl --data '{"chunkFootprint": { "bottomLeft": { "x": 0, "y": 0 }, "sideLength": 256 }, "planetRarity":16384, "planetHashKey": 8}' -H "Content-Type: application/json" -X POST localhost:8000/mine`

You can get far more debug logs by running with `RUST_LOG=trace cargo run`

### Performance tuning

By defualt the miner uses as much of your processor as possible. This is good if you're on a cloud machine, but not great if you're also trying to run the game. A single processor machine with 4 cores, and 2 threads per core, has 8 threads available. Adjust for your machine, but you might try 4 or 6 threads and see if you dont get even a little better performance because its not competing with the game for resources. You can set the amount of threads to use with an environment variable before the executable like `RAYON_NUM_THREADS=4 mimc-fast`

