# REzASM
## A Rust port of [EzASM](https://github.com/ezasm-org/ezasm/tree/main)
"Rewrite it in Rust," and so we did.

## Introduction
The goal of this project is to create a small-instruction-set assembly-like programming language interpreter written in Rust. 
We will ship an IDE-like GUI interface for programming, running code, and inspecting the current state of the environment. 
This simple interpreted language would be able to demonstrate the concepts of a lower level assembly language while still being simple to write. 
The instructions would be intuitive and simple compared to MIPS (e.g., no system calls or immediate limits) and act upon registers akin to other assembly languages.

## Getting Started (Windows)
1. Ensure you have installed [Rust](https://www.rust-lang.org/learn/get-started) and [Node.js](https://nodejs.org/en/download)
2. Download the [Visual Studio Installer](https://visualstudio.microsoft.com/downloads/) and install the Desktop Development with C++ Workload
3. Install cargo-tauri using `cargo install tauri-cli wasm-pack` in a terminal emulator
4. Install tailwindcss using `npm install -g tailwindcss` in a terminal emulator
5. Clone this repository to your system
6. Open this repository in your terminal emulator
7. Run `npm i` in a terminal emulator to install the node dependencies
8. Run the application in development mode using `cargo tauri dev`

## Getting Started (Linux)
1. Ensure you have installed [Rust](https://www.rust-lang.org/learn/get-started) and [Node.js](https://nodejs.org/en/download)
2. Install cargo-tauri using `cargo install tauri-cli wasm-pack`
3. Install tailwindcss using `npm install -g tailwindcss`
4. Clone this repository to your system
5. Open this repository in your terminal
6. Run `npm i` in your terminal to install the node dependencies
7. Run the application in development mode using `cargo tauri dev`

## Getting Started (MacOS)
**Under Construction / May not work yet**
1. Ensure you have installed [Rust](https://www.rust-lang.org/learn/get-started) and [Node.js](https://nodejs.org/en/download)
2. Install cargo-tauri using `cargo install tauri-cli wasm-pack`
3. Install tailwindcss using `npm install -g tailwindcss`
4. Clone this repository to your system
5. Open this repository in your terminal
6. Run `npm i` in your terminal to install the node dependencies
7. Run the application in development mode using `cargo tauri dev`
