import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  // pass your local crate path to the plugin
  plugins: [
    wasmPack('./src-wasm')
  ],
  server: {
    open: true // Automatically open the app in the browser
  }
});