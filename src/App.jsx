import {useCallback, useEffect, useReducer, useRef, useState} from "react";
import RegistryView from "./components/RegistryView.jsx";
import WorkerPromise from "webworker-promise";
import init from "../wasm/rezasm_wasm.js";
import {RUST} from "./rust_functions.js";

import "../dist/output.css";
import MemoryView from "./components/MemoryView.jsx";
import Console from "./components/Console.jsx";
import Controls from "./components/Controls.jsx";
import Editor from "./components/Editor.jsx";

const STATE = {
    IDLE: 1,
    LOADING: 2,
    LOADED: 3,
    RUNNING: 4,
    PAUSED: 5,
    STOPPED: 6,
};

const CALLBACKS_TRIGGERS = {
    RESET: "RESET",
    STEP: "STEP"
};

const CALLBACK_TYPES = {
    CONSOLE: "CONSOLE",
    MEMORY: "MEMORY",
    REGISTRY: "REGISTRY",
};


let initialCallbacks = {};
Object.values(CALLBACKS_TRIGGERS).map(x => initialCallbacks[x] = {});

function App() {
    const [code, setCode] = useState("");
    const [error, setError] = useState("");
    const [exitCode, setExitCode] = useState("");
    const state = useRef(STATE.IDLE);
    const timerId = useRef(null);
    const [instructionDelay, setInstructionDelay] = useState(5);
    const [wasmLoaded, setWasmLoaded] = useState(false);

    const callbacks = useRef(initialCallbacks);
    const [, forceUpdate] = useReducer(() => Date.now());

    const setState = (newState) => {
        state.current = newState;
        forceUpdate();
    };

    const disallowExecution = useCallback(() => {
        if (timerId.current !== null) {
            clearTimeout(timerId.current);
            timerId.current = null;
        }
    }, []);

    const callStepCallbacks = useCallback(() => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.STEP]).map(callback => callback());
    }, []);

    const callResetCallbacks = useCallback( () => {
        Object.values(callbacks.current[CALLBACKS_TRIGGERS.RESET]).map(callback => callback());
    }, []);

    const registerCallback = useCallback((trigger, type, callback) => {
        callbacks.current[trigger][type] = callback;
    }, []);

    const isErrorState = useCallback(() => {
        return error !== "";
    }, [error]);

    const setErrorState = useCallback(newState => {
        setError(newState);
        setState(STATE.STOPPED);
    }, []);

    const isCompleted = useCallback(async () => {
        return await RUST.IS_COMPLETED({});
    }, []);

    const getExitStatus = useCallback(async () => {
        return await RUST.GET_EXIT_STATUS({});
    }, []);

    const stop = useCallback(async currentState => {
        disallowExecution();
        await RUST.STOP({});
        currentState = STATE.STOPPED;
        setState(currentState);
        return currentState;
    }, []);

    const reset = useCallback(async () => {
        disallowExecution();
        await RUST.RESET({});
        setState(STATE.IDLE);
        callStepCallbacks();
        callResetCallbacks();
        setExitCode("");
        setError("");
        return STATE.IDLE;
    }, [callResetCallbacks, callStepCallbacks]);

    const load = useCallback(async () => {
        if (state.current < STATE.LOADED) {
            await RUST.LOAD({lines: code})
                .then(() => {
                    setState(STATE.LOADED);
                })
                .catch(error => {
                    setErrorState(error);
                    setState(STATE.STOPPED);
                });
        }
    }, [code, setErrorState, state]);

    const checkAndHandleProgramCompletion = useCallback(async () => {
        callStepCallbacks();
        if (await isCompleted() || isErrorState()) {
            disallowExecution();
            setState(STATE.STOPPED);
            setExitCode("" + await getExitStatus());
            return true;
        } else {
            return false;
        }
    }, [callStepCallbacks, getExitStatus, isCompleted, isErrorState]);

    const handleStepCall = useCallback(async () => {
        RUST.STEP({})
            .then(async () => await checkAndHandleProgramCompletion())
            .catch(error => {
                setErrorState(error);
                setState(STATE.STOPPED);
            });
    }, [checkAndHandleProgramCompletion, setErrorState]);

    const step = useCallback(async () => {
        if (state.current< STATE.LOADED) {
            await reset();
            await load();
            if (state.current !== STATE.STOPPED && ! await checkAndHandleProgramCompletion()) {
                return handleStepCall().then(() => setState(STATE.PAUSED));
            }
        } else if (state.current === STATE.PAUSED || state.current === STATE.RUNNING) {
            return handleStepCall();
        }
    }, [checkAndHandleProgramCompletion, handleStepCall, load, reset, state]);

    const recursivelyCallStep = useCallback(async () => {
        if (state.current === STATE.STOPPED) {
            return;
        }
        checkAndHandleProgramCompletion().then(async completed => {
            if (!completed && state.current === STATE.RUNNING) {
                handleStepCall().then(() => {
                    timerId.current = setTimeout(recursivelyCallStep, instructionDelay);
                }).catch((e) => {
                    timerId.current = null;
                    setErrorState(e);
                    setState(STATE.STOPPED);
                });
            }
        });
    }, [checkAndHandleProgramCompletion, handleStepCall, instructionDelay, setErrorState, state]);

    const run = useCallback(() => {
        if (state.current !== STATE.STOPPED) {
            setState(STATE.RUNNING);
            recursivelyCallStep();
        }
    }, [recursivelyCallStep, state]);

    useEffect(() => {
        if (init) {
            window.__WASM_DEFINED__ = true;
            window.worker = new WorkerPromise(new Worker("/src/worker.js", { type: "module" }));
            window.worker.postMessage({command: "ping"}).then((e) => {
                if (e !== "pong") {
                    throw Error("Could not communicate with the web-worker");
                }
                setWasmLoaded(true);
            });
        }
    }, []);

    return (
        <div className="container">
            <div className="fill">
                <Controls state={state} setState={setState} run={run} stop={stop} step={step} reset={reset} load={load} isErrorState={isErrorState}/>
                <div className="mt-2 mb-2 row codearea">
                    <Editor state={state} setCode={setCode} />
                    <RegistryView loaded={wasmLoaded} registerCallback={registerCallback} />
                </div>
            </div>
            <div className="fill">
                <MemoryView loaded={wasmLoaded} registerCallback={registerCallback} />
            </div>
            <div className="fill">
                <Console loaded={wasmLoaded} registerCallback={registerCallback} exitCode={exitCode} error={error} />
            </div>
        </div>
    );
}

export default App;

export {STATE, CALLBACKS_TRIGGERS, CALLBACK_TYPES};
