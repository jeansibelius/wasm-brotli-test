# Brotli compression WebAssembly module with Rust

A simple [WebAssembly (wasm)](https://developer.mozilla.org/en-US/docs/WebAssembly) module that allows compression with Brotli.

References:

- https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
- https://rustwasm.github.io/book/

## Building

For web targets: `wasm-pack build --target web`

For e.g. Cloudflare Workers: `wasm-pack build --target bundler`

NB: To use this in Cloudflare Workers, a JavaScript shim is required as detailed in
the [Cloudflare Workers Rust documentation](https://developers.cloudflare.com/workers/runtime-apis/webassembly/rust/#javascript-plumbing-wasm-bindgen).

## Serving the example `index.html`

`python3 -m http.server`
