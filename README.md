# 🦀 Rust, WebAssembly, and Snowpack

> ✨ Bootstrapped with Create Snowpack App (CSA).

## Quick Start

Ensure Rust is installed on your machine. For more information, check out the [Rust docs](https://www.rust-lang.org/tools/install).

```
# Pull template into a working directory
npx create-snowpack-app my-awesome-app --template snowpack-template-ts-rust-wasm --use-yarn

# Install required Rust crates
cd my-awesome-app
cargo install cargo-watch wasm-pack

# Manually build and test WebAssembly module
yarn build:wasm && yarn test:wasm
```

### Includes:

- Typescript Support
- Building and testing Rust/WebAssembly modules
- Uses [snowpack-plugin-wasm-pack](https://www.npmjs.com/package/snowpack-plugin-wasm-pack) for HMR
- Prettier for code linting and formatting

### Additional Resources (Rust/WebAssembly):

- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/) (book)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/introduction.html) (docs)
- [WebAssembly By Example](https://wasmbyexample.dev/home.en-us.html#) (examples)

## Available Scripts

### yarn build:wasm

Changes into Rust crate and builds WebAssembly into `pkg/` and `target/` directories.

### yarn test:wasm

Executes Rust tests bound to the WebAssembly module. Configured to run in a node environment.

**Note:** Modify the directories in the scripts above after creating your own crate.

### yarn start

Runs the app in the development mode.
Open http://localhost:8080 to view it in the browser.

The page will reload if you make edits.
You will also see any lint errors in the console.

### yarn build

Builds a static copy of your site to the `build/` folder.
Your app is ready to be deployed!

**For the best production performance:** Add a build bundler plugin like [@snowpack/plugin-webpack](https://github.com/snowpackjs/snowpack/tree/main/plugins/plugin-webpack) or [snowpack-plugin-rollup-bundle](https://github.com/ParamagicDev/snowpack-plugin-rollup-bundle) to your `snowpack.config.mjs` config file.

### Q: What about Eject?

No eject needed! Snowpack guarantees zero lock-in, and CSA strives for the same.
