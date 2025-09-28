const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

function components() {
  const components = {};
  for (const component of ['app', 'questions']) {
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
    filename: '[name].[contenthash].js',
    chunkFilename: '[name].[contenthash].js',
    clean: true,
  },
  resolve: {
    extensions: ['.ts', '.js'],
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
