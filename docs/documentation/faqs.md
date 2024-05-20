# Frequently Asked Questions

## Does this mean that holochain already supports mobile?

Well, not quite. Let's break it down to the two main mobile platforms:

### Android

Holochain has experimental support for Android. This means that holochain works as expected on Android, **except for these issues**:

- [Go compiler issue on Android 11 or later](https://github.com/holochain/tx5/issues/87). This means that in these Android versions, the device can't communicate with anyone on the network, so in practicality holochain does not work.
- [Installation of apps takes more than 40 seconds to complete on an average Android device](https://github.com/holochain/holochain/issues/3243).
- [Every time the Android app is opened, holochain takes ~10 seconds to boot up, so there is a long loading screen](https://github.com/holochain/holochain/issues/3243).

### iOS

In development, holochain works as expected in iOS. But Apple prevents JIT compilation in iOS devices, so when a holochain app is published in the iOS store, it does not work. Thankfully there is already [work in progress done by wasmer](https://github.com/wasmerio/wasmer/issues/4486) to address this issue. Stay tuned for updates!

---

## Well, okey... Then how does tauri-plugin-holochain help me now?

For now, you can build a desktop only executable hApp that your users can download and use. It's ready to be deployed and iterated upon. After the issues with holochain mobile outlined above are resolved, you will be able to upgrade to a new version of the plugin to automatically get mobile support in your hApp.

This is the way ourselves in darskoil studio are building hApps right now. We are monitoring the issues at the technical level, and in constant communication with the core holochain development team. At the same time, while they get resolved, we are building the MVP version for our hApps.
