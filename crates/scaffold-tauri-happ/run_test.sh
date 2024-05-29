#!/usr/bin/bash
set -e

DIR=$(pwd)

nix shell --override-input versions "github:holochain/holochain?dir=versions/0_3_rc" github:holochain/holochain#hc-scaffold --command bash -c "
cd /tmp
rm -rf forum-scaffold-tauri-happ

hc-scaffold --template lit web-app forum-scaffold-tauri-happ --setup-nix true -F
cd /tmp/forum-scaffold-tauri-happ
nix flake update
nix develop --command bash -c \"npm i && hc scaffold dna forum && hc scaffold zome posts --integrity dnas/forum/zomes/integrity/ --coordinator dnas/forum/zomes/coordinator/\"
"

nix run --accept-flake-config  .#scaffold-tauri-happ -- --path /tmp/forum-scaffold-tauri-happ --ui-package ui --bundle-identifier org.myorg.myapp

cd /tmp/forum-scaffold-tauri-happ

nix develop --override-input p2p-shipyard $DIR --command bash -c "
set -e

npm i
npm run tauri icon $DIR/examples/end-user-happ/src-tauri/icons/icon.png
npm run build:happ
npm run tauri build -- --no-bundle
"

nix develop --override-input p2p-shipyard $DIR .#androidDev --command bash -c "
set -e

npm i
npm run tauri android init
TAURI_ANDROID_PACKAGE_NAME_PREFIX=myorg TAURI_ANDROID_PACKAGE_NAME_APP_NAME=mypackage cargo build --target aarch64-linux-android
"
