/// <reference types="vitest" />
import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import tsconfigPaths from 'vite-tsconfig-paths';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig({
  plugins: [
    preact(),
    wasm(),
    topLevelAwait(),
    tsconfigPaths(),
    ...(process.env.VITEST
      ? []
      : [
          VitePWA({
            registerType: 'autoUpdate',
            manifest: false,
            includeAssets: ['icons/*.png', 'manifest.webmanifest'],
            workbox: {
              globPatterns: ['**/*.{js,css,html,svg,woff2,wasm}'],
              maximumFileSizeToCacheInBytes: 3 * 1024 * 1024,
            },
          }),
        ]),
  ],
  test: {
    environment: 'node',
  },
});
