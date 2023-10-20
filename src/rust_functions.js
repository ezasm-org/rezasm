import {invoke} from "@tauri-apps/api/tauri";
import {
    wasm_get_exit_status, wasm_get_register_names, wasm_get_register_value, wasm_get_register_values,
    wasm_is_completed,
    wasm_load,
    wasm_reset,
    wasm_step,
    wasm_stop
} from "../dist/wasm/rezasm_wasm.js";

const callWorkerFunction = message => {
    return new Promise((resolve, reject) => {
        window.worker.postMessage(message)
            .then(result => resolve(result))
            .catch(e => {
                console.log("Error calling command: " + message.command);
                reject(e.message);
            });
    });
};

const rust_load = async lines => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_load", {lines});
    } else if (wasm_load) {
        return callWorkerFunction({command: "load", argument: lines});
    } else {
        throw new Error("Function load does not exist");
    }
};

const rust_step = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_step", {});
    } else if (wasm_step) {
        return callWorkerFunction({command: "step"});
    } else {
        throw new Error("Function step does not exist");
    }
};

const rust_reset = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_reset", {});
    } else if (wasm_reset) {
        return callWorkerFunction({command: "reset"});
    } else {
        throw new Error("Function reset does not exist");
    }
};

const rust_stop = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_stop", {});
    } else if (wasm_stop) {
        return callWorkerFunction({command: "stop"});
    } else {
        throw new Error("Function stop does not exist");
    }
};

const rust_is_completed = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_is_completed", {});
    } else if (wasm_is_completed) {
        return callWorkerFunction({command: "is_completed"});
    } else {
        throw new Error("Function is_completed does not exist");
    }
};

const rust_get_exit_status = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_exit_status", {});
    } else if (wasm_get_exit_status) {
        return callWorkerFunction({command: "get_exit_status"});
    } else {
        throw new Error("Function get_exit_status does not exist");
    }
};

const rust_get_register_value = async register => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_register_value", {register});
    } else if (wasm_get_register_value) {
        return callWorkerFunction({command: "get_register_value", argument: register});
    } else {
        throw new Error("Function get_register_value does not exist");
    }
};

const rust_get_register_names = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_register_names", {});
    } else if (wasm_get_register_names) {
        return callWorkerFunction({command: "get_register_names"});
    } else {
        throw new Error("Function get_register_names does not exist");
    }
};

const rust_get_register_values = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_register_values", {});
    } else if (wasm_get_register_values) {
        return callWorkerFunction({command: "get_register_values"});
    } else {
        throw new Error("Function get_register_values does not exist");
    }
};

const rust_get_memory_bounds = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_memory_bounds", {});
    } else if (wasm_get_register_values) {
        return callWorkerFunction({command: "get_memory_bounds"});
    } else {
        throw new Error("Function get_memory_bounds does not exist");
    }
};

const rust_get_memory_slice = async (address, length) => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_memory_slice", {address, length});
    } else if (wasm_get_register_value) {
        return callWorkerFunction({command: "get_memory_slice", argument: {address: address, length: length}});
    } else {
        throw new Error("Function get_memory_slice does not exist");
    }
};

const rust_get_word_size = async () => {
    if (window.__TAURI_IPC__) {
        return invoke("tauri_get_word_size", {});
    } else if (wasm_get_register_values) {
        return callWorkerFunction({command: "get_word_size"});
    } else {
        throw new Error("Function get_word_size does not exist");
    }
};

export {
    rust_reset,
    rust_load,
    rust_stop,
    rust_step,
    rust_is_completed,
    rust_get_exit_status,
    rust_get_memory_bounds,
    rust_get_memory_slice,
    rust_get_register_names,
    rust_get_register_value,
    rust_get_register_values,
    rust_get_word_size
};
