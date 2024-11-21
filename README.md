# Portfolio

[![Release to Github Pages](https://github.com/fubblea/portfolio/actions/workflows/gh-pages-deploy.yml/badge.svg)](https://github.com/fubblea/portfolio/actions/workflows/gh-pages-deploy.yml)

## Local Development

### Pre-requisites

If you don't have Rust nightly, you can install it with
```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```

### Develop

To develop your Leptos CSR project, running

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.


### Build

To build a Leptos CSR app for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

