import WorkerPromise from "webworker-promise";

declare global {
    interface Window {
        __WASM_DEFINED__?: boolean;
        worker: WorkerPromise;
    }
}
