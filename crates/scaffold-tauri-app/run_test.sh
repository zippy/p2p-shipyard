#!/usr/bin/bash
set -e

DIR=$(pwd)

nix shell --override-input versions "github:holochain/holochain?dir=versions/0_3_rc" github:holochain/holochain#hc-scaffold --command bash -c "
cd /tmp
rm -rf forum-scaffold-tauri-app

hc-scaffold --template lit web-app forum-scaffold-tauri-app --setup-nix true -F
cd /tmp/forum-scaffold-tauri-app
nix flake update
nix develop --command bash -c \"npm i && hc scaffold dna forum && hc scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/\"
"

nix run --accept-flake-config  .#scaffold-tauri-app -- --path /tmp/forum-scaffold-tauri-app --ui-package ui --bundle-identifier org.myorg.myapp

cd /tmp/forum-scaffold-tauri-app

nix develop --override-input p2p-shipyard $DIR --command bash -c "
set -e

npm i
npm run tauri build
"
nix develop --override-input p2p-shipyard $DIR\#androidDev --command bash -c "
set -e

npm i
npm run tauri android init
npm run tauri android build
"
