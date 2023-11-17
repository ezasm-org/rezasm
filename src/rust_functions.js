import {invoke} from "@tauri-apps/api/tauri";

const setsEqual = (xs, ys) => xs.size === ys.size && [...xs].every((x) => ys.has(x));

const callWorkerFunction = message => {
    return new Promise((resolve, reject) => {
        window.worker.postMessage(message)
            .then(result => resolve(result))
            .catch(e => {
                console.log(e.message);
                reject(e.message);
            });
    });
};

const isWasmLoaded = () => {
    return callWorkerFunction({command: "status"});
};

// name is the name of the function in rust (without "tauri_" or "wasm_")
// shape is an array describing the keys that are expected to be defined in props
const get_rust_function = (name, shape) => {
    shape = shape ?? [];
    const shapeSet = new Set(shape);
    return async (props) => {
        props = props ?? {};
        if (!setsEqual(shapeSet, new Set(Object.keys(props)))) {
            throw new Error(`Function '${name} passed with unexpected shape'`);
        }
        if (window.__TAURI_IPC__) {
            return invoke(`tauri_${name}`, props);
        } else {
            while (! await isWasmLoaded()) {
                // wait
            }
            return callWorkerFunction({command: name, argument: props, shape: shape});
        }
    };
};

const RUST = {
    LOAD: get_rust_function("load", ["lines"]),
    STEP: get_rust_function("step"),
    RESET: get_rust_function("reset"),
    STOP: get_rust_function("stop"),
    IS_COMPLETED: get_rust_function("is_completed"),
    GET_EXIT_STATUS: get_rust_function("get_exit_status"),
    GET_REGISTER_VALUE: get_rust_function("get_register_value", ["register"]),
    GET_REGISTER_NAMES: get_rust_function("get_register_names"),
    GET_REGISTER_VALUES: get_rust_function("get_register_values"),
    GET_MEMORY_BOUNDS: get_rust_function("get_memory_bounds"),
    GET_MEMORY_SLICE: get_rust_function("get_memory_slice", ["address", "length"]),
    GET_WORD_SIZE: get_rust_function("get_word_size"),
    RECEIVE_INPUT: get_rust_function("receive_input", ["data"]),
};

export {
    RUST
};
