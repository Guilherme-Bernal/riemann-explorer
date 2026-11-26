import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// base './' faz o build funcionar tanto local quanto no GitHub Pages (/riemann-explorer/).
export default defineConfig({
  base: './',
  plugins: [vue()],
  server: { proxy: { '/api': 'http://127.0.0.1:3000' } }
})