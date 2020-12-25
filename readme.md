# Dark forest mimc miner

[Dark Forest](https://zkga.me/), the world's first decentralized real-time
strategy game. Built on Ethereum with zkSNARKsis a https://zkga.me/

This rust package pulls out the fog of war miner to its own binary providing
faster results than the built in javascript miner on the same machine. It also
can run remotely in the cloud or even a rapsberry pi 4 has been found to provide
good results.

# Install

* Install [rust for your operating system](https://www.rust-lang.org/tools/install) probably with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* The rocket webserver requires nightly so add that `rustup install nightly`
* Run it `cargo run`
* Connect to it with an in game plugin like [RemoteExplorePlugin.js](https://github.com/phated/df-plugins)

# Troubleshooting

To test its working you can do a quick curl `curl --data '{"chunkFootprint": {
"bottomLeft": { "x": 0, "y": 0 }, "sideLength": 256 }, "planetRarity":16384}' -H
"Content-Type: application/json" -X POST localhost:8000/mine`

Remember to pause the built in miner if you're running it on the same machine as
the game client or theyll just fight eachother.

If its taking too much performance from the rest of your desktop experience you
can try running with limited cores `RAYON_NUM_THREADS=4 cargo run`