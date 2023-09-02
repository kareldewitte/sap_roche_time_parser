const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: ['./index.js'],
    //mode: 'production',
    
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
   
    
    module: {
        rules: [
          {
            test: /\.css$/i,
            include: path.resolve(__dirname, 'css'),
            use: ["style-loader", "css-loader","postcss-loader"],
          },
            {
              test: /\.png$/,
              include: path.resolve(__dirname, 'assets'),
              use: 'file-loader'
            }

        ],
      },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html'
        }),
    
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
   }
};