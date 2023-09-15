import registerWebworker from "webworker-promise/lib/register";
import init from "../dist/wasm/rezasm_wasm.js";
import {
    wasm_load, wasm_step, wasm_stop, wasm_reset, wasm_is_completed, wasm_get_exit_status, wasm_get_register_value
} from "../dist/wasm";

init().then(() => {
    console.log("WebAssembly code loaded");
});

registerWebworker(async (message, emit) => {
    const command = message.command;
    const data = message.argument;

    try {
        if (command === "ping") {
            return "pong";
        } else if (command === "load") {
            if (data === undefined) {
                throw "Call to 'load' without providing string data";
            }
            return wasm_load(data);
        } else if (command === "step") {
            return wasm_step();
        } else if (command === "stop") {
            return wasm_stop();
        } else if (command === "reset") {
            return wasm_reset();
        } else if (command === "is_completed") {
            return wasm_is_completed();
        } else if (command === "get_exit_status") {
            return wasm_get_exit_status();
        } else if (command === "get_register_value") {
            if (data === undefined) {
                throw "Call to 'get_register_value' without providing string data";
            }
            return wasm_get_register_value(data);
        } else {
            throw `Invalid command: '${command}'`;
        }
    } catch (error) {
        console.log(error);
        throw new Error(error);
    }
});
