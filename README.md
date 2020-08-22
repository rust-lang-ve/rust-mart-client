<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars3.githubusercontent.com/u/68873317?s=120&v=4" height="120" width="120" />
  </div>
  <h1 align="center">yew-rust-mart</h1>
  <span align="center">🛒 Simple e-commerce SPA made with Yew</span>
</div>

<div align="center">
  <img src="https://raw.githubusercontent.com/rust-lang-ve/yew-rust-mart/main/docs/image.png" width="550" height="375" />
</div>

## Running Locally

### Install required packages

```bash
cargo install wasm-pack
cargo install cargo-make
cargo install simple-http-server
```

### Build the project

Theres 2 ways available to build the project, either using `wasm-pack` only or by using `wasm-back`
through the `make` task.

```bash
# using wasm-pack only
bash ./build.sh

# using the make task
cargo make build
```

### Running the Simple HTTP Server

After building the project, you can run an HTTP server to serve the files. Is important to note
that the HTTP Server in question must have support for **wasm**.

If you have installed the `simple-http-server` then you can run the following, in the current directory:

```bash
cd ./static/ && simple-http-server
```

And then, open your browser in http://localhost:8000/

## Motivation

Follow [Sheshbabu Chinnakonda](http://www.sheshbabu.com/posts/rust-wasm-yew-single-page-application/)'s post
on **Single Page Applications using Rust** to learn more about Rust for the Front-End development.

[Sheshbabu GitHub](https://github.com/sheshbabu)
