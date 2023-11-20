import registerWebworker from "webworker-promise/lib/register";

let wasmFunctions = {};

if (self.__WASM_LOADED__ === undefined && self.document === undefined) {
    self.__WASM_LOADED__ = false;
    import("../wasm/rezasm_wasm.js").then(async (module) => {
        await module.default();
        wasmFunctions = Object.entries(module)
            .filter(([x,]) => x.startsWith("wasm_"))
            .reduce((left, [x, y]) => {
                left[x] = y;
                return left;
            }, {});
        self.__WASM_LOADED__ = true;
        console.log("WebAssembly code loaded");
    }).catch((error) => {
        console.log("WebAssembly could not load", error);
    });

    const worker = registerWebworker(async (message, emit) => {
        const command = message.command;
        if (command === "status") {
            return self.__WASM_LOADED__;
        }
        const data = message.argument ?? {};
        const shape = message.shape ?? [];
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

    self.emitPrintString = (string) => {
        worker.emit("wasm_print", string);
    };
}
