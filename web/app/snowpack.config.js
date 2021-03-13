/** @type {import("snowpack").SnowpackUserConfig } */

const httpProxy = require('http-proxy');
const proxy = httpProxy.createServer({ target: 'http://localhost:3000/' });

module.exports = {
  mount: {
    public: {url: '/', static: true},
    src: {url: '/dist'},
  },
  plugins: [
    '@snowpack/plugin-svelte',
    '@snowpack/plugin-dotenv',
    '@snowpack/plugin-webpack'
  ],
  routes: [
    /* Example: Enable an SPA Fallback in development: */
    // {"match": "routes", "src": ".*", "dest": "/index.html"},
    {
      match: "routes",
      src: '^\/$',
      dest: (req, res) => {
        proxy.web(req,res);
      }
    },
    {
      match: "routes",
      src: '^\/[a-zA-Z0-9-_]{22}$',
      dest: (req, res) => {
        console.log(res);
        proxy.web(req,res);
      }
    },
  ],
  optimize: {
    /* Example: Bundle your final build: */
    // "bundle": true,
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    /* ... */
  },
  buildOptions: {
    /* ... */
  },
};
