import registerWebworker from "webworker-promise/lib/register";

let wasmFunctions = {};

let wasmLoaded = false;

import("../dist/wasm/rezasm_wasm.js").then(async (module) => {
    wasmLoaded = false;
    await module.default();
    wasmFunctions = Object.entries(module)
        .filter(([x,]) => x.startsWith("wasm_"))
        .reduce((left, [x, y]) => {
            left[x] = y;
            return left;
        }, {});
    wasmLoaded = true;
    console.log("WebAssembly code loaded");
}).catch((error) => {
    console.log("WebAssembly could not load", error);
});

// use this function before calling a wasm function to ensure wasm has loaded
const waitForWasmLoad = () => {
    while(!wasmLoaded) {}
};

registerWebworker(async (message, emit) => {
    const command = message.command;
    const data = message.argument ?? {};
    const shape = message.shape ?? [];
    console.log(data);
    const functionArguments = shape.map(arg => {
        const argument = data[arg];
        if (argument !== undefined) {
            return argument;
        } else {
            throw new Error(`Function '${command}' passed without required argument ${arg}`);
        }
    });

    const wasmFunction = wasmFunctions[`wasm_${command}`];

    if (command === "ping") {
        return "pong";
    } else if (wasmFunction) {
        try {
            return wasmFunction(...functionArguments);
        } catch (error) {
            throw new Error(error);
        }
    } else {
        throw new Error(`Invalid command: '${command}'`);
    }
});

export { waitForWasmLoad };
