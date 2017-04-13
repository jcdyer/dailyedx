var path = require('path');

module.exports = {
  entry: './frontend/src/index.jsx',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'frontend/build')
  },
  module: {
    loaders: [
        {
            test: /\.jsx?/,
            include: path.resolve(__dirname, 'frontend/src'),
            loader: 'babel-loader'
        }
    ]
  }
};