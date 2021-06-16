/**
 * The wasm-pack bundle exports 'init' by default, call this once before using any other function
 * snowpack-plugin-wasm-pack automatically configures under the name of the package
 *
 * Once you're ready to build, clear the file. Happy Hacking!
 */
import init, { greet, add } from 'wasm_crate';

const main = async () => {
  await init();

  greet('Welcome to Snowpack, Rust, and WebAssembly!');

  console.log('Addition from Rust: ', add(4, 2));
};

main().catch((err) => {
  console.error(err);
});
