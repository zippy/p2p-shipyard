# Getting to know Tauri

## What is Tauri?

Tauri is a rust-based engine that allows the creation of cross-platform apps that have a web frontend and a rust-based backend. As such, it is an alternative to electron, or flutter, that can target both mobile and desktop platforms.

Although it's still young, tauri already has a [wide ecosystem of plugins that enable common desktop and mobile use cases](https://github.com/tauri-apps/plugins-workspace).

You can learn more about it [in the official Tauri documentation](https://beta.tauri.app/concepts/).

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

After the initial set up and scaffolding, the initial tauri app can only be built for desktop apps. To enable mobile support, there is a bit more work that needs to be done.

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARNING]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine...

