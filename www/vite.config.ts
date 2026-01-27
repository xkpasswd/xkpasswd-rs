import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';
import wasm from 'vite-plugin-wasm';
import tsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
  plugins: [preact(), wasm(), tsconfigPaths()],
});

