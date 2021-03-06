const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  devServer: {
    open: true,
    liveReload: true,
    watchContentBase: true
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js"
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin(["index.html"])]
};
