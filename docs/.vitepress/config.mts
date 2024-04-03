import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Tauri Plugin Holochain",
  description: "Cross-platform holochain runtimes and apps",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Examples', link: '/markdown-examples' }
    ],

    sidebar: [
      {
        text: 'Guides',
        items: [
          { text: 'How to create an executable hApp ', link: '/how-to-create-an-executable-happ' },
          { text: 'How to create a holochain runtime', link: '/how-to-create-a-holochain-runtime' },
          { text: 'Setting up the android platform', link: '/android-setup' },
        ]
      },
      {
        text: 'Troubleshooting',
        link: '/troubleshooting'
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/darksoil-studio/tauri-plugin-holochain' }
    ]
  }
})
