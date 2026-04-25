import { defineConfig } from "vitepress";

export default defineConfig({
  title: "AK-Macros",
  description: "Script-like Rust macros for productive automation",
  lastUpdated: true,
  themeConfig: {
    nav: [
      { text: "Guide", link: "/guide/introduction" },
      { text: "API", link: "/api/macros" },
      { text: "Architecture", link: "/architecture" },
      { text: "GitHub", link: "https://github.com/hamdymohamedak/AK-macro/" }
    ],
    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Introduction", link: "/guide/introduction" },
          { text: "Installation", link: "/guide/installation" },
          { text: "Quick Start", link: "/guide/quick-start" },
          { text: "Enterprise Usage", link: "/guide/enterprise-usage" },
          { text: "Migration from 0.x", link: "/guide/migration" }
        ]
      },
      {
        text: "Reference",
        items: [
          { text: "Macros", link: "/api/macros" },
          { text: "Error Model", link: "/api/errors" }
        ]
      },
      {
        text: "Project",
        items: [{ text: "Architecture", link: "/architecture" }]
      }
    ],
    socialLinks: [
      { icon: "github", link: "https://github.com/hamdymohamedak/AK-macro/" }
    ]
  }
});
