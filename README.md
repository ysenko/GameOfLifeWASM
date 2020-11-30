# Simple Game of life WASM implemented in Rust

![Build Status](https://github.com/ysenko/GameOfLifeWASM/workflows/.github/workflows/wasm-test-actions.yml/badge.svg?branch=main)

## Running locally
1. Make sure you have Rust and its toolchain installed locally: https://www.rust-lang.org/tools/install
2. Install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/
3. Build wasm extension, by running `wasm-pack build` from the root folder of the project
4. Install NodeJS and npm.
5. Run `npm install` inside the `www` folder. This will install all JS dependencies.
6. Finally run `npm run start` inside the `www` folder. This will start a local development web server.
7. Your GameOfLife in noow accessible at `http://localhost:8080/`

## Runinng tests
Run `wasm-pack test --headless --firefox` in the root folder of the prroject.
