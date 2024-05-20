## Developing for Android

While developing a hApp, it's not that useful to just have one agent to test your hApp with. Instead, you usually need a couple of peers to be able to interact with one another. 

The scaffolding setup 

::: code-group
```bash [npm]
npm run android:network
```

```bash [yarn]
yarn android:network
```

```bash [pnpm]
pnpm android:network
```
:::

```bash
adb devices
```
