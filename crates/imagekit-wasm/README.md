# Imagekit-wasm

Requirements:

- `wasm32-unknown-unknown` rust target
- `wasm-bindgen-cli` installed
- `binaryen` installed for `wasm-opt` and `wasm2js`

```sh
$ rustup target add wasm32-unknown-unknown
$ cargo install -f wasm-bindgen-cli
$ brew install binaryen
```

## Build

```sh
# dev
$ ./build.sh
# release
$ ./build.sh -r
```
