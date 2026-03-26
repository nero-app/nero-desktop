import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  server: {
    port: 1420,
    strictPort: true,
  },
  cacheDir: "../node_modules/.vite",
});
