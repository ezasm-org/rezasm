/* eslint-disable no-unused-vars */
import {invoke} from "@tauri-apps/api/tauri";
import WorkerPromise from "webworker-promise";
import workerURL from "./worker.ts?worker&url";
import {type Message, type ValidWasmCommandStrings} from "./worker.ts";

const setsEqual = <T>(xs: Set<T>, ys: Set<T>) => xs.size === ys.size && [...xs].every((x) => ys.has(x));

const isWasmLoaded = () => {
    return callWorkerFunction({command: "status"});
};

const loadWasm = async () => {
    return import("../wasm/rezasm_wasm.js").then(() => {
        if (!window.__WASM_DEFINED__) {
            window.__WASM_DEFINED__ = true;
            window.worker = new WorkerPromise(new Worker(workerURL, { type: "module" }));
            window.worker.postMessage({command: "ping"}).then((e) => {
                return e === "pong";
            });
        } else {
            return true;
        }
    });
};

const callWorkerFunction = (message: Message) => {
    return new Promise((resolve, reject) => {
        window.worker.postMessage(message)
            .then(result => resolve(result))
            .catch(e => {
                console.log(e.message);
                reject(e.message);
            });
    });
};

// name is the name of the function in rust (without "tauri_" or "wasm_")
// shape is an array describing the keys that are expected to be defined in props
export const get_rust_function = (name: ValidWasmCommandStrings, shape?: string[]) => {
    shape = shape ?? [];
    const shapeSet = new Set(shape);
    return async (props: Record<string, unknown>) => {
        props = props ?? {};
        if (!setsEqual(shapeSet, new Set(Object.keys(props)))) {
            throw new Error(`Function '${name} passed with unexpected shape'`);
        }
        // @ts-expect-error -- This is not always going to exist, but the compiler doesn't know that
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

export interface RustFunctions {
    LOAD: (
        props: {lines: string}
    ) => Promise<void>;
    STEP: (
        props: Record<string, never>
    ) => Promise<void>;
    STEP_BACK: (
        props: Record<string, never>
    ) => Promise<void>;
    RESET: (
        props: Record<string, never>
    ) => Promise<void>;
    STOP: (
        props: Record<string, never>
    ) => Promise<void>;
    IS_COMPLETED: (
        props: Record<string, never>
    ) => Promise<boolean>;
    GET_EXIT_STATUS: (
        props: Record<string, never>
    ) => Promise<bigint>;
    GET_REGISTER_VALUE: (
        props: {register: string}
    ) => Promise<bigint | undefined>;
    GET_REGISTER_NAMES: (
        props: Record<string, never>
    ) => Promise<string[]>;
    GET_REGISTER_VALUES: (
        props: Record<string, never>
    ) => Promise<BigInt64Array>;
    GET_MEMORY_BOUNDS: (
        props: Record<string, never>
    ) => Promise<BigInt64Array>;
    GET_MEMORY_SLICE: (
        props: {address: number, length: number}
    ) => Promise<BigInt64Array>;
    GET_WORD_SIZE: (
        props: Record<string, never>
    ) => Promise<number>;
    RECEIVE_INPUT: (
        props: {data: string}
    ) => Promise<void>;
}

const RUST = {
    LOAD: get_rust_function("load", ["lines"]),
    STEP: get_rust_function("step"),
    STEP_BACK: get_rust_function("step_back"),
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
} as RustFunctions;

export {
    RUST,
    loadWasm
};
