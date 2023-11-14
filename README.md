# REzASM
## A Rust port of [EzASM](https://github.com/ezasm-org/ezasm/tree/main)
"Rewrite it in Rust," and so we did.

## Introduction
The goal of this project is to create a small-instruction-set assembly-like programming language interpreter written in Rust. 
We will ship an IDE-like GUI interface for programming, running code, and inspecting the current state of the environment. 
This simple interpreted language would be able to demonstrate the concepts of a lower level assembly language while still being simple to write. 
The instructions would be intuitive and simple compared to MIPS (e.g., no system calls or immediate limits) and act upon registers akin to other assembly languages.

## Getting Started
1. Ensure you have installed [Rust](https://www.rust-lang.org/learn/get-started) and [Node.js](https://nodejs.org/en/download)
2. Complete the [Tauri Prerequisites Installation](https://tauri.app/v1/guides/getting-started/prerequisites/#installing) process
3. Install tauri using `cargo install tauri-cli wasm-pack` in a terminal emulator
4. Clone this repository to your system
5. Open this repository in your terminal
6. Update npm using `npm install -g npm@latest`
7. Run `npm install` to install the node dependencies
8. Run the test suite to ensure everything works using `cargo test`
9. Run the CLI application by using `cargo run`
10. Run the tauri application in development mode using `cargo tauri dev`
11. Test the WebAssembly code by going to http://localhost:1420 while the tauri application is open
