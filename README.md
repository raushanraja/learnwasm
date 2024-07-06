# learnwasm

### Prerequisite:
- `Rust`
- `npm`

### Rust Package Requirements:
- wasm-pack : `cargo install wasm-pack`
- wasm-bindgen


### Setup 1.
- Create a new rust lib package
- Add lib to the Cargo.toml 
- Build for web using wasm-pack command
- Import js file from the built pkg into html & call the wasm method using js 

```text
// Create a new rust lib project
cargo new --lib hello-wasm

// Add required lib to Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

// Add wasm-bindgen to rust project
cargo add wasm-bindgen

// Build for web
wasm-pack build --target web

// Import the js file in index.html and  use the methods.

// Serve index.html, like: python -m http.server and open the url
```
