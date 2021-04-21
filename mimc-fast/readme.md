# Dark forest mimc miner

[Dark Forest](https://zkga.me/), the world's first decentralized real-time
strategy game. Built on Ethereum with zkSNARKsis a <https://zkga.me/>

Note if you're not technical and cant get through this guide, remember you can
get a lot of speedup by just having the stock game use more cores with
something like `df.setMinerCores(16)` in the console. So try that first.

This rust package pulls out the fog of war miner to its own binary with a
webserver for in game plugins to talk to it. It provides faster results than the
built in javascript miner on the same machine. If you're using it on the same
machine as the game Pause your in game miner as theyll just compete with
eachother and give WORSE results. It also can run remotely in the
cloud or even a rapsberry pi 4 has been found to provide 1-2k hashes.

## Install

The rust miner on the same machine as your game can be faster than the javascript because it can more fully utilize the processor. But if you're running on the same machine as the game, pause the in game miner. They're just going to compete with eachother and give you worse peformance.

- Install [rust for your operating system](https://www.rust-lang.org/tools/install) probably with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Install with `cargo install --git https://github.com/jacobrosenthal/mimc-fast --branch v6-tide-split`
- Run it with `mimc-fast`
- Connect to it with an in game plugin like [RemoteExplorePlugin.js](https://github.com/darkforest-eth/plugins/tree/master/content/productivity/remote-explore)

### google cloud run

Be warned this costs money. Google apparently offer a bunch of free credits to
start though so this can be cheap for a while. I offer no support for this so
don't post questions about this cloud service, or any others. PRs are welcome to
make the docker files more accomodating though.

- Fork the plugin to your own repo on Github
- Make a new project on Google Cloud Run
- Go to <https://console.cloud.google.com/run>
- Click "+ Create Service"
- Choose the server you wanna use and the name of the service
- Choose "continuously deploy from source repository
- Click "setup with cloud build" and then connect it to the repo
- Change the advanced settings - port is 8000 and then change the memory/cpus according to what you want (2gb memory, 4cpu has been reported as ~4k hashes)
- Setup trigger and then start it

## Troubleshooting

To test its working you can do a quick curl `curl --data '{"chunkFootprint": { "bottomLeft": { "x": 0, "y": 0 }, "sideLength": 256 }, "planetRarity":16384, planetHashKey":8}' -H "Content-Type: application/json" -X POST localhost:4433/mine`

Remember to pause the built in miner if you're running it on the same machine as
the game client or theyll just fight eachother.

If its taking too much performance from the rest of your desktop experience you
can try running with limited cores `RAYON_NUM_THREADS=4 cargo run`
