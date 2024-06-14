import WorkerPromise from "webworker-promise";

declare global {
    interface Window {
        __WASM_DEFINED__?: boolean;
        __WASM_LOADED__?: boolean;
        worker: WorkerPromise;
        // eslint-disable-next-line no-unused-vars
        emitPrintString?: (string: string) => void;
    }
}
