import { defineConfig } from "astro/config";
import react from "@astrojs/react";
// astro.config.mjs
export default defineConfig({
  // ... other configurations
  buildOptions: {
    site: 'http://localhost:3000',
  },

  integrations: [react()],

  vite: {
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `@import "src/styles/global.css";`,
        },
      },
    },
  }
});