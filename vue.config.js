const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  configureWebpack: {
    plugins: [
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, 'tinygl-renderer'),
        outDir: path.resolve(__dirname, 'pkg')
      })
    ],
    watchOptions: {
      ignored: ['target', 'node_modules']
    }
  }
}
