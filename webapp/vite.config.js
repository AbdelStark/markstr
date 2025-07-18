import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill'
import { NodeModulesPolyfillPlugin } from '@esbuild-plugins/node-modules-polyfill'
import rollupNodePolyFill from 'rollup-plugin-polyfill-node'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default ({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')

  return defineConfig({
    plugins: [react(), wasm(), topLevelAwait()],
    resolve: {
      alias: {
        buffer: 'rollup-plugin-node-polyfills/polyfills/buffer-es6',
        stream: 'rollup-plugin-node-polyfills/polyfills/stream',
        events: 'rollup-plugin-node-polyfills/polyfills/events',
        util: 'rollup-plugin-node-polyfills/polyfills/util',
      },
      dedupe: ['react', 'react-dom']
    },
    define: {
      __RPC_URL__: JSON.stringify(env.VITE_RPC_URL || ''),
      __RPC_PORT__: JSON.stringify(env.VITE_RPC_PORT || ''),
      __RPC_USER__: JSON.stringify(env.VITE_RPC_USER || ''),
      __RPC_PASSWORD__: JSON.stringify(env.VITE_RPC_PASSWORD || ''),
      __ORACLE_RPC_WALLET__: JSON.stringify(env.VITE_ORACLE_RPC_WALLET || ''),
      __ALICE_RPC_WALLET__: JSON.stringify(env.VITE_ALICE_RPC_WALLET || ''),
      __BOB_RPC_WALLET__: JSON.stringify(env.VITE_BOB_RPC_WALLET || ''),
      __CHARLIE_RPC_WALLET__: JSON.stringify(env.VITE_CHARLIE_RPC_WALLET || ''),
      __BITCOIN_NETWORK__: JSON.stringify(env.VITE_BITCOIN_NETWORK || ''),
      __EXPLORER_API_BASE__: JSON.stringify(env.VITE_EXPLORER_API_BASE || ''),
    },
    optimizeDeps: {
      exclude: ['tiny-secp256k1'],
      esbuildOptions: {
        define: {
          global: 'globalThis',
        },
        plugins: [
          NodeGlobalsPolyfillPlugin({
            process: true,
            buffer: true,
          }),
          NodeModulesPolyfillPlugin(),
        ],
      },
    },
    build: {
      rollupOptions: {
        plugins: [rollupNodePolyFill()],
      },
    },
    server: {
      port: 3001,
      open: true,
      proxy: {
        '/wallet': {
          target: `http://${env.VITE_RPC_URL}:${env.VITE_RPC_PORT}`,
          changeOrigin: true,
          auth: `${env.VITE_RPC_USER}:${env.VITE_RPC_PASSWORD}`,
          secure: false,
        }
      }
    },
  })
}