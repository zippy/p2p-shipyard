# How to create an executable hApp

This guide describes how to create a hApp that can be directly installed and executed by the end users, for **both desktop and mobile platforms**.

## Motivation

The [scaffolding tool](https://github.com/holochain/scaffolding) is a great way to create and package holochain applications. However, its built-in templates don't produce an end-user installable application. They produce a `.webhapp` file, that needs to be installed in a holochain runtime, which is the actual app that is being executed in the OS of the end-user (eg. the [launcher](https://github.com/holochain/launcher)).

We need a way to create end-users applications for mobile platforms to create simple experiences similar to what users are used to in the existing app stores. 

> [!NOTE]
> This is also what [kangaroo](https://github.com/holochain-apps/holochain-kangaroo) accomplishes. However, the approach that kangaroo takes is to serve as a template for you to clone it. The approach for tauri-plugin-holochain is just to be another tauri plugin, which means that apps will get bug fixes and new features automatically when upgrading to a new version of the plugin.

## Scaffolding the tauri app

0. [Scaffold your hApp using the scaffolding tool](https://developer.holochain.org/get-started/3-forum-app-tutorial/).

> [!NOTE]
> If you already have a hApp that you want to convert to a tauri executable app, you can skip this step.

1. Run this command inside the repository of your web-hApp:

```bash
nix run github:darksoil-studio/tauri-plugin-holochain#scaffold-tauri-app
```

And follow along to answer all the necessary prompts.

This will execute all the required steps to convert your previously scaffolded hApp to an end-user executable tauri app.

2. Take a look into the files that the scaffold command edited, and adapt them if necessary:

- `flake.nix`: added the `tauri-plugin-holochain` input and its `devShells`.
- `package.json`: added set up scripts and some `devDependencies`.
- `ui/vite.config.ts`: set the server configuration necessary for tauri.
- `src-tauri`: here is where the code for the backend of the tauri app lives.
  - The tauri app will just use the UI that the scaffolding tool produced as its own UI.

> [!WARNING]
> The `scaffold-tauri-app` command assumes that you have scaffolded your app using the scaffolding tool.
>
> It also tries to make smart guesses about the structure of your project, but it can be tricky to support every repository structure. Please open an issue in the github repository if you find any bugs in it!

That's it! We now have a fully functional end-user executable hApp. 

> [!WARNING]
> The scaffolded tauri app is missing icons, which are needed for the app to compile. Run through the rest of this guide and the following one ("Getting to know Tauri") to be able to generate the icons for your Tauri app.

## Development Environment

The `scaffold-tauri-app` has added the necessary nix `devShells` to your `flake.nix` file so that you don't need to follow install anything to get the tauri or Android development environment.

> [!NOTE]
> Nix `devShells` are packages that describe development environments, with all their dependencies and environment variables, so that the developer does not need to configure manually their setup.

As usual, run this command to enter the development environment:

```bash
nix develop
```

This can take a while while it builds all the required dependencies.

Next, run these commands:

::: code-group
```bash [npm]
npm install
npm start
```

```bash [yarn]
yarn install
yarn start
```

```bash [pnpm]
pnpm install
pnpm start
```
:::

This will start two agents connected to each other.

Under the hood, these commands are running tauri CLI commands. As such, we should get to know Tauri a bit better to be comfortable while developing the app. Go to [Getting to know Tauri](./getting-to-know-tauri.md) to familiarize yourself with it.

