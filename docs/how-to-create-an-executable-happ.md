# How to create an executable hApp

This guide describes how to create a hApp that can be directly installed and executed by the end users, for **both desktop and mobile platforms**.

## Motivation

The [scaffolding tool](https://github.com/holochain/scaffolding) is a great way to create and package holochain applications. However, its built-in templates don't produce an end-user installable application. They produce a `.webhapp` file, that needs to be installed in a holochain runtime, which is the actual app that is being executed in the OS of the end-user (eg. the [launcher](https://github.com/holochain/launcher)).

We need a way to create end-users applications for mobile platforms to create simple experiences similar to what users are used to in the existing app stores. 

> [!NOTE]
> This is also what [kangaroo](https://github.com/holochain-apps/holochain-kangaroo) accomplishes. However, the approach that kangaroo takes is to serve as a template for you to clone it. The approach for tauri-plugin-holochain is just to be another tauri plugin, which means that apps will get bug fixes and new features automatically when upgrading to a new version of the plugin.

## Creating an executable hApp

### Scaffolding the hApp 

**If you are starting from scratch**, the best way to begin is by scaffolding the app using the scaffolding tool, by running: 

```bash
nix run github:darksoil-studio/tauri-plugin-holochain#hc-scaffold -- web-app
```

> [!NOTE]
> This command uses the holochain scaffolding tool with a custom template, that scaffolds everything that we need to get started.
> It is using a custom template based on the [holochain-open-dev custom template](https://github.com/holochain-open-dev/templates), which uses [Lit](https://lit.dev) as its frontend framework. Please get in touch with us if you would like to have it based on another framework.

### Converting an existing hApp into an executable hApp

**If you already have an existing hApp**, you can run these commands:



--- 

That's it! We have created a fully functional executable hApp. 

It is in fact just a Tauri app that depends on `tauri-plugin-holochain`. As such, we should get to know Tauri a bit better to be comfortable while developing the app. Go to [Getting to know Tauri](./getting-to-know-tauri.md) to familiarize yourself with it.

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARNING]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine. Work is already in progress, so stay tuned! You can learn more by reading the [FAQs](/faqs).
