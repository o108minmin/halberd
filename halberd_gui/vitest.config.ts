import { defineConfig } from "vitest/config";

export default defineConfig({
  esbuild: {
    jsx: "automatic",
  },
  test: {
    environment: "jsdom",
    environmentOptions: {
      jsdom: {
        url: "http://localhost:1420",
      },
    },
    globals: true,
    setupFiles: ["./src/test/setup.ts"],
  },
});
