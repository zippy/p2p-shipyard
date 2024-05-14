# Getting to know Tauri

## What is Tauri?

Tauri is a rust-based engine that allows the creation of cross-platform apps that have a web frontend and a rust-based backend. As such, it is an alternative to electron, or flutter, that can target both mobile and desktop platforms.

Although it's still young, tauri already has a [wide ecosystem of plugins that enable common desktop and mobile use cases](https://github.com/tauri-apps/plugins-workspace).

You can learn more about it [in the official Tauri documentation](https://beta.tauri.app/concepts/).

## Backend

As we just learned, the backend for Tauri is written in rust. Let's understand how that backend works, so that you can edit its behavior to suit your needs, if necessary.

It's important that you take a look at the file `src-tauri/src/lib.rs`. This is now the main starting point for your hApp, which includes the `tauri-plugin-holochain` plugin. This plugin will run holochain under the hood, and converts the Tauri app in to a full holochain runtime.

Still in that file, take a closer look at the `.setup()` hook. This is the initialization code that will be run when your end-user app is executed. You can see that the scaffolded code already contains a simple initialization logic, that you can extend to any need you have to 

Refer to the [rust documentation for the `tauri-plugin-holochain`](https://docs.rs/tauri-plugin-holochain) to learn all the commands that the plugin offers.

## CLI

Tauri includes a powerful CLI that allows us to execute the different commands we need in our development lifecycle:

- To start a development version of our app, run:

::: code-group
```bash [npm]
npm run tauri dev
```

```bash [yarn]
yarn tauri dev
```

```bash [pnpm]
pnpm tauri dev
```
:::

- To create a production build for the current platform, run:

::: code-group
```bash [npm]
npm run tauri build
```

```bash [yarn]
yarn tauri build
```

```bash [pnpm]
pnpm tauri build
```
:::

- See all the available CLI commands with:

::: code-group
```bash [npm]
npm run tauri 
```

```bash [yarn]
yarn tauri 
```

```bash [pnpm]
pnpm tauri
```
:::

And learn more about the CLI in the [official Tauri guide](https://beta.tauri.app/references/v2/cli/).

## Mobile

After the initial set up and scaffolding, our tauri app can only be built for desktop apps. To enable mobile support, there is a bit more work that needs to be done.

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARNING]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine...

