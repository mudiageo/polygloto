import { svelteTesting } from "@testing-library/svelte/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [sveltekit(), tailwindcss()],
  clearScreen: false,

  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },

  test: {
    workspace: [{
      extends: "./vite.config.js",
      plugins: [svelteTesting()],

      test: {
        name: "client",
        environment: "jsdom",
        clearMocks: true,
        include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
        exclude: ['src/lib/server/**'],
        setupFiles: ['./vitest-setup-client.js']
      }
    }, {
      extends: "./vite.config.js",

      test: {
        name: "server",
        environment: "node",
        include: ['src/**/*.{test,spec}.{js,ts}'],
        exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
      }
    }]
  }
});
