import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Tauri Plugin Holochain",
  description: "Cross-platform holochain runtimes and apps",
  base: "/tauri-plugin-holochain",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Documentation", link: "/how-to-create-an-executable-happ" },
      { text: "License", link: "/license" },
    ],

    sidebar: [
      {
        text: "Guides",
        items: [
          {
            text: "How to create an executable hApp ",
            link: "/how-to-create-an-executable-happ",
          },
          {
            text: "How to create a holochain runtime",
            link: "/how-to-create-a-holochain-runtime",
          },
          { text: "Getting to know Tauri", link: "/getting-to-know-tauri" },
          { text: "Setting up Android", link: "/android-setup" },
        ],
      },
      {
        text: "FAQs",
        link: "/faqs",
      },
      {
        text: "Troubleshooting",
        link: "/troubleshooting",
      },
    ],

    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/darksoil-studio/tauri-plugin-holochain",
      },
    ],
  },
});
