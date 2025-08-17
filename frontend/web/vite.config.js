import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 3000,
    host: true,
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    assetsDir: 'assets',
    rollupOptions: {
      output: {
        manualChunks: {
          wasm: ['../pkg/rnes_web.js'],
        },
      },
    },
  },
  optimizeDeps: {
    exclude: ['../pkg/rnes_web.js'],
  },
  assetsInclude: ['**/*.wasm'],
  publicDir: 'public',
  resolve: {
    alias: {
      '@': '/src',
    },
  },
});
