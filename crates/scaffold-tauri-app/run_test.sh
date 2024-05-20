#!/usr/bin/bash
set -e

DIR=$(pwd)

nix shell github:holochain/holochain#hc-scaffold --command bash -c "
cd /tmp
rm -rf forum-scaffold-tauri-app

hc-scaffold web-app --template lit forum-scaffold-tauri-app --setup-nix true
cd /tmp/forum-scaffold-tauri-app
nix flake update
nix develop --command bash -c \"npm i\"
"

nix run .#scaffold-tauri-app -- --path /tmp/forum-scaffold-tauri-app --ui-package ui

cd /tmp/forum-scaffold-tauri-app

nix develop --override-input tauri-plugin-holochain $DIR --command bash -c "
set -e

npm i
npm run build:happ
npm run tauri build
"
