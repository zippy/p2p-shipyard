# How to create an executable hApp

This guide describes how to create a hApp that can be directly installed and executed by the end users, for **both desktop and mobile platforms**.

## Motivation

The [scaffolding tool](https://github.com/holochain/scaffolding) is a great way to create and package holochain applications. However, its built-in templates don't produce an end-user installable application, rather they produce a `.webhapp` file, that needs to be installed in a holochain runtime or similar.

We need a way to create end-users applications for mobile platforms to create simple experiences similar to what users are used to in the existing app stores. 

> [!NOTE]
> This is also what [kangaroo](https://github.com/holochain-apps/holochain-kangaroo) accomplishes. The approach it takes is to serve as a template for you to clone it, instead the approach for tauri-plugin-holochain is just to be another tauri plugin.

## Creating the hApp 

```bash
nix run github:darksoil-studio/tauri-plugin-holochain#hc-scaffold -- web-app
```

## Mobile

### Android

Continue to the [Android setup](./android-setup.md);

### iOS 

> [!WARN]
> Coming soon! Holochain working on iOS is blocked by wasmer having an interpreter wasm engine...
