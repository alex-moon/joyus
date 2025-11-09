const path = require('path');
const webpack = require('webpack');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

function components() {
  const components = {};
  for (const component of ['app', 'joy_form', 'joy_cards', 'joy_card']) {
    components[`css/component/${component}`] = path.resolve(__dirname, 'src', 'component', component, `${component}.scss`);
  }
  return components;
}

module.exports = {
  entry: {
    bundle: path.resolve(__dirname, 'src', 'index.ts'),
    ...components(),
  },
  output: {
    path: path.resolve(__dirname, 'public', 'assets'),
    filename: '[name].js',
    chunkFilename: '[name].js',
    clean: true,
  },
  resolve: {
    extensions: ['.ts', '.js'],
    alias: {
      '@engine': path.resolve(__dirname, 'src/datastar/engine'),
      '@plugins': path.resolve(__dirname, 'src/datastar/plugins'),
      '@bundles': path.resolve(__dirname, 'src/datastar/bundles'),
      '@utils': path.resolve(__dirname, 'src/datastar/utils')
    },
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.s[ac]ss$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader', 'sass-loader'],
      },
    ],
  },
  plugins: [
    new webpack.DefinePlugin({
      ALIAS: JSON.stringify(null),
    }),
    new MiniCssExtractPlugin({ filename: '[name].css' }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'src', 'index.html'),
      filename: '../index.html',
      inject: 'body',
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, 'src', 'assets'),
          to: '.',
          noErrorOnMissing: true,
        },
      ],
    }),
  ],
  devtool: 'source-map',
};
