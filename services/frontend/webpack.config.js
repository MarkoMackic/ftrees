const path = require('path');
const webpack = require("webpack");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
    entry: {
        home: ['./src/homepage/home.js'],
    },
    resolve: {
        // see below for an explanation
        alias: {
            svelte: path.resolve('node_modules', 'svelte'),
		},
        extensions: ['.mjs', '.js', '.svelte', '.css', '.scss'],
        mainFields: ['svelte', 'browser', 'module', 'main'],
        modules: [
            path.resolve(__dirname, 'node_modules'),
            path.resolve(__dirname, 'src'),
        ]
    },
    output: {
        filename: '[name].bundle.js',
        path: path.resolve(__dirname, 'public/')
    },
    plugins: [
        //new WebpackCleanupPlugin(),
        new MiniCssExtractPlugin({
            filename: '[name].bundle.css',
        }),
        new HtmlWebpackPlugin({
            chunks: ['home'],
            filename: 'index.html'
        }),
    ],
    module: {
        rules: [
            {
                test: /\.(?:svelte|m?js)$/,
                exclude: /(node_modules|bower_components)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env'],
                        plugins: [
                            'babel-plugin-transform-es2015-template-literals',
                            "@babel/plugin-syntax-top-level-await",
                            "@babel/plugin-proposal-class-properties",
                            [
                                "@babel/plugin-transform-runtime",
                                {
                                    "corejs": 2,
                                }
                            ]
                        ]
                    }
                }
            },
            {
                type: 'javascript/auto',
                test: /\.json$/,
                use: [
                    {
                      loader: 'file-loader',
                      options: {
                          name: "./json/[name].[ext]"
                      }
                    }
                ]
            },
            {
                test: /\.(html|svelte)$/,
                use: {
                  loader: 'svelte-loader',
                  options: {
                    compilerOptions: {
                        dev: true
                    },
                    emitCss: false
                  }
                }
            },
            {
                // required to prevent errors from Svelte on Webpack 5+, omit on Webpack 4
                test: /node_modules\/svelte\/.*\.mjs$/,
                resolve: {
                  fullySpecified: false
                }
            },
            {
                test: /\.(sa|sc|c)ss$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    'css-loader',
                    {
                        loader: 'sass-loader',
                        options: {
                          sassOptions: {
                            includePaths: [
                              './src',
                              './node_modules'
                            ]
                          }
                        }
                    },
                ],
            },
            {
                test: /\.(woff(2)?|ttf|eot|svg|png)(\?v=\d+\.\d+\.\d+)?$/,
                use: [
                    {
                        loader: 'file-loader',
                        options: {
                            name: '[name].[ext]',
                            outputPath: 'assets/',
                            publicPath: url => 'assets/' + url
                        }
                    }
                ]
            }
        ],
    },
    experiments : {
        topLevelAwait : true
    },
    mode: 'development'
}