#!/usr/bin/bash
set -e

nix shell github:holochain/holochain#hc-scaffold --command bash -c "
cd /tmp
rm -rf forum-scaffold-tauri-app

hc-scaffold web-app --template lit forum-scaffold-tauri-app --setup-nix true
cd /tmp/forum-scaffold-tauri-app
nix flake update
nix develop --command bash -c \"npm i\"
"

nix run .#scaffold-tauri-app -- forum-scaffold-tauri-app --path /tmp/forum-scaffold-tauri-app

cd /tmp/forum-scaffold-tauri-app

nix flake update
nix develop --command bash -c "
set -e

npm i
npm run build:happ
npm run tauri build
"
