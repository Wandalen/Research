# Helpfull tools to create a web extension in Rust

## Crates

### [wasm-bindgen](https://crates.io/crates/wasm-bindgen)

This is the main crate for creating web plugins in Rust. It provides a bridge between JavaScript and WebAssembly, allowing Rust functions to be called from JavaScript and vice versa. It also provides tools for binding to browser APIs.

### [web-sys](https://crates.io/crates/web-sys)

This crate provides access to all standard browser APIs at the WebIDL level. It allows you to use JavaScript browser APIs in Rust code and use Rust code in JavaScript code.

This crate by default contains very little when compiled as almost all of its exposed APIs are gated by Cargo features. The exhaustive list of features can be found in `crates/web-sys/Cargo.toml`, but the rule of thumb for `web-sys` is that each type has its own cargo feature (named after the type). Using an API requires enabling the features for all types used in the API, and APIs should mention in the documentation what features they require.

### [js-sys](https://crates.io/crates/js-sys)

This crate allows you to use JavaScript APIs in Rust code without the need to use WebIDL. It provides raw bindings to JS global APIs for projects using wasm-bindgen. This crate is handwritten and intended to work in all JS environments like browsers and Node.js.

## Guide

You can find general documentation about using Rust and WebAssembly together [here](https://rustwasm.github.io/docs/wasm-bindgen/).

## Examples

### [wasm extension template](https://github.com/Mubelotix/wasm-extension-template)

An easy-to-use template for Rust web extensions. The Rust code is compiled to WASM and ran as a content script.

This template allows you to generate a "Hello World" web extension running a Rust program compiled to WASM.
The program will be executed as a content script, without the need of being injected into the page.

Supports both manifest v2 and v3.

### [rust web extension talk](https://github.com/grasegger/rust-web-extension-talk)

The skeleton of a web extension, that can manipulate websites to another liking. Including a small dive into a comfortable build setup for a full-blown web extension.

You can find more information about this project in this [video](https://www.youtube.com/watch?v=m8DqHZK27X0)

### [Addonis](https://github.com/TheAdnan/Addonis)

Firefox add-on skeleton generator made in Rust.
