import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path'
import vueJsx from '@vitejs/plugin-vue-jsx'
import monacoEditorPlugin from 'vite-plugin-monaco-editor-esm';
import svgLoader from 'vite-svg-loader'
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(), 
    vueJsx(),
    svgLoader(),
    monacoEditorPlugin(),
  ],
  build: {
    target: ['edge90', 'chrome90', 'firefox90', 'safari15'],
  },
  resolve: {
    alias: {
      '@': path.resolve('./src'),
    }
  },
  optimizeDeps: {
    include: [
      'monaco-editor/esm/vs/language/json/json.worker',
      'monaco-editor/esm/vs/language/css/css.worker',
      'monaco-editor/esm/vs/language/html/html.worker',
      'monaco-editor/esm/vs/language/typescript/ts.worker',
      'monaco-editor/esm/vs/editor/editor.worker',
    ],
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
