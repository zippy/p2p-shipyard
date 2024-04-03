# Getting to know Tauri

## What is Tauri

Tauri is a rust-based engine that allows the creation of cross-platform apps that have a web frontend and a rust-based backend. As such, it is an alternative to electron, or flutter.

You can learn more about it [here](https://beta.tauri.app/concepts/).

## CLI

Tauri includes a powerful CLI that allows us to execute the different commands we need in our development lifecycle:

- To start a development version of our app, run:

```bash
pnpm tauri dev
```

- To create a production build for the current platform, run:

```bash
pnpm tauri build
```

- See all the commands available to you with:

```bash
pnpm tauri
```

And learn more about the CLI in the [official Tauri guide](https://beta.tauri.app/references/v2/cli/).

## Mobile

The default tauri app can be built only for desktop apps. To enable mobile support, there is a bit more work that needs to be done.

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARNING]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine...

