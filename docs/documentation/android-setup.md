# Android Setup

> [!NOTE]
> This guide assumes that you have already gone through either [how to create an executable hApp](./happ-setup.md) or [how to create a holochain runtime](./runtime-setup.md).

1. In the root folder of your repository, run:

::: code-group
```bash [npm]
npm run tauri android init
```

```bash [yarn]
yarn tauri android init
```

```bash [pnpm]
pnpm tauri android init
```
:::

This should initialize all the necessary android files for your app.

2. Go in the `src-tauri/gen/android/app/build.gradle.kts` that was generated in the previous step, and set the "usesCleartextTraffic" to true:

```kotlin
plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

android {
    compileSdk = 33
    namespace = "com.tauri.tauri_app"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false" // [!code --]
        manifestPlaceholders["usesCleartextTraffic"] = "true" // [!code ++]
        applicationId = "com.tauri.tauri_app"
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.6.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.8.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")
```

3. In your Android device, enable the [developer options](https://developer.android.com/studio/debug/dev-options).

4. After you have enabled the developer options, [enable USB debbuging](https://developer.android.com/studio/debug/dev-options#Enable-debugging).

5. Connect your Android device to your computer with a USB cable, and confirm in your Android device that you allow USB debugging from this computer.

6. In the root folder of your repository, run:

```bash
nix develop .#androidDev
```

This is a replacement command for the usual `nix develop`, which includes `Android Studio`, and all the necessary tooling that you need for Android development. Every time you want to test or build for the Android platform, you will need to enter the nix devShell this way and then your command from inside of it.

> [!WARNING]
> The first time this is run, it will take some time. This is because nix has to download and build all the necessary Android tooling. After the first time, it will be almost instant.

7. Inside your `androidDev` devShell, run:

```bash
adb devices
```

If all the previous steps were successful, you should see your device in the list of devices.

8. Verify that everything is working by running the app for android with:

::: code-group
```bash [npm]
npm run tauri android dev
```

```bash [yarn]
yarn tauri android dev
```

```bash [pnpm]
pnpm tauri android dev
```
:::

--- 

That's it! You have completed the setup for the Android platform.
