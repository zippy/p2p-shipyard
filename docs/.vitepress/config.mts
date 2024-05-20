import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Tauri Plugin Holochain",
  description: "Cross-platform holochain runtimes and apps",
  base: "/tauri-plugin-holochain",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      {
        text: "Documentation",
        link: "/documentation/how-to-create-an-executable-happ",
      },
      { text: "License", link: "/license/license" },
    ],

    sidebar: {
      "/documentation/": [
        {
          text: "Guides",
          items: [
            {
              text: "How to create an executable hApp ",
              link: "/documentation/how-to-create-an-executable-happ",
            },
            {
              text: "How to create a holochain runtime",
              link: "/documentation/how-to-create-a-holochain-runtime",
            },
            {
              text: "Getting to know Tauri",
              link: "/documentation/getting-to-know-tauri",
            },
            {
              text: "Android",
              items: [
                {
                  text: "Project Setup",
                  link: "/documentation/android/project-setup",
                },
                {
                  text: "Device Setup",
                  link: "/documentation/android/device-setup",
                },
                {
                  text: "Developing",
                  link: "/documentation/android/developing",
                },
                {
                  text: "Building for Production",
                  link: "/documentation/android/building-for-production",
                },
              ],
            },
          ],
        },
        {
          text: "FAQs",
          link: "/documentation/faqs",
        },
        {
          text: "Troubleshooting",
          link: "/documentation/troubleshooting",
        },
      ],
      "/license": [],
    },

    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/darksoil-studio/tauri-plugin-holochain",
      },
    ],
  },
});
