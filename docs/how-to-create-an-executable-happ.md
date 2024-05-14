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

1. Run this command inside the repository of your web-app:

```bash
nix run github:darksoil-studio/tauri-plugin-holochain#scaffold-tauri-app
```

And follow along to answer all the necessary prompts.

This will execute all the required steps to convert your previously scaffolded hApp to an end-user executable tauri app. The command tries to guess as best as possible what's in your project.

> [!WARNING]
> The `scaffold-tauri-app` command assumes that you have scaffolded your app using the scaffolding tool.

> [!WARNING]
> The `scaffold-tauri-app` command tries to make smart guesses about the structure of your project, but it can be tricky to support every repository structure. Please open an issue in the github repository if you find any bugs in it!

2. Take a look into the files that the scaffold command edited, and adapt them if necessary:

- `flake.nix`
- `package.json`

--- 

That's it! We have created a fully functional executable hApp. 

It is in fact just a Tauri app that depends on `tauri-plugin-holochain`. As such, we should get to know Tauri a bit better to be comfortable while developing the app. Go to [Getting to know Tauri](./getting-to-know-tauri.md) to familiarize yourself with it.

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARNING]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine. Work is already in progress, so stay tuned! You can learn more by reading the [FAQs](/faqs).
