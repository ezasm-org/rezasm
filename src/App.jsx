import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { wasm_initialize_backend, wasm_load, wasm_run, wasm_step, wasm_stop, wasm_reset, wasm_is_completed,
         wasm_get_exit_status, wasm_get_register_value } from "../dist/wasm";
import init from "../dist/wasm/rezasm_wasm.js"
import "../dist/output.css";

const STATE = {
    IDLE: 1,
    LOADED: 2,
    RUNNING: 3,
    PAUSED: 4,
    STOPPED: 5,
}

const isSome = option => {
    return option !== null;
}

const isNone = option => {
    return option === null;
}

const rust_load = async lines => {
    if (window.__TAURI__) {
        return invoke("tauri_load", {lines});
    } else if (wasm_load) {
        return wasm_load(lines);
    } else {
        throw "Load does not exist";
    }
}

function App() {
    const [lines, setLines] = useState("");
    const [error, setError] = useState("");
    const [result, setResult] = useState("");
    const [state, setState] = useState(STATE.IDLE);

    const isErrorState = useCallback(() => {
        return error !== "";
    }, [error]);

    const clearErrorState = useCallback(() => {
        setError("");
    }, []);

    const setErrorState = useCallback(newState => {
        setError(newState);
        setState(STATE.STOPPED);
    }, []);

    const getErrorState = useCallback(() => {
        return error;
    }, [error]);

    const isCompleted = useCallback(async () => {
        return await invoke("tauri_is_completed", {});
    }, []);

    const getExitStatus = useCallback(async () => {
        return await invoke("tauri_get_exit_status", {});
    }, []);

    const getRegisterValue = useCallback(async register => {
        return await invoke("tauri_get_register_value", {register});
    }, []);

    const reset = useCallback(async () => {
        await invoke("tauri_reset", {});
        setState(STATE.IDLE);
        setResult("");
        clearErrorState();
        return STATE.IDLE;
    }, [clearErrorState]);

    const load = useCallback(async (currentState) => {
        if (currentState >= STATE.LOADED) {
            return currentState;
        }
        await rust_load(lines)
            .then(() => {
                currentState = STATE.LOADED;
                setState(currentState);
            })
            .catch(error => {
                setErrorState(error);
                currentState = STATE.STOPPED;
                setState(currentState);
            });

        return currentState;
    }, [setErrorState, lines]);

    const run = useCallback(async currentState => {
        currentState = await reset();
        currentState = await load(currentState);
        setState(currentState);

        if (currentState >= STATE.LOADED) {
            if (await isCompleted()) {
                setResult("Program exited with exit code " + await getExitStatus());
                currentState = STATE.STOPPED;
                setState(currentState);
            } else {
                currentState = STATE.RUNNING;
                setState(currentState);
                await invoke("tauri_run", {});
            }
        }
        return currentState;
    }, [reset, load, getExitStatus, isCompleted]);

    const step = useCallback(async currentState => {
        if (currentState < STATE.LOADED) {
            currentState = await load(currentState);
            currentState = STATE.PAUSED;
            setState(currentState);
        }

        if (currentState >= STATE.LOADED && currentState !== STATE.STOPPED) {
            if (await isCompleted()) {
                setResult("Program exited with exit code " +  await getExitStatus());
                currentState = STATE.STOPPED;
                setState(currentState);
            } else {
                currentState = STATE.PAUSED;
                setState(currentState);
                await invoke("tauri_step", {});
            }
        }
        return currentState
    }, [load, isErrorState, getExitStatus, isCompleted]);

    const stop = useCallback(async currentState => {
        await invoke("tauri_stop", {})
        currentState = STATE.STOPPED;
        setState(currentState);
        return currentState;
    }, []);

    useEffect(() => {
        window.errorCallback = error => {
            console.log(error);
            setErrorState(error);
        };

        window.programCompletionCallback = exitStatus => {
            setResult("Program exited with exit code " + exitStatus);
            setState(STATE.STOPPED);
        }
    }, [state]);

    useEffect(() => {
        if (init) {
            init().then(r => {
                wasm_initialize_backend();
            });
        }
    }, [])

    return (
        <div className="container">
            <h1><b>Welcome to rezasm!</b></h1>
            <div className="mt-2 mb-2 row">
                    { state === STATE.RUNNING ?
                    <button className="btn-operation bg-red-500 hover:bg-red-700" onClick={(e) => {
                        stop(state);
                    }}>
                        Stop
                    </button>
                    :
                    <button className="btn-operation bg-green-500 hover:bg-green-700" onClick={(e) => {
                        run(state);
                    }}>
                        Start
                    </button>
                }

                { state === STATE.PAUSED ?
                    <button className="btn-operation bg-emerald-600 hover:bg-emerald-700" onClick={(e) => {
                        // TODO resume
                    }}>
                        Resume
                    </button>
                    :
                    <button className="btn-operation bg-cyan-600 hover:bg-cyan-700"
                            disabled={state !== STATE.RUNNING}
                            onClick={(e) => {
                        // TODO pause
                    }}>
                        Pause
                    </button>
                }

                <button className="btn-operation bg-blue-500 hover:bg-blue-700"
                        disabled={(state !== STATE.PAUSED && state !== STATE.IDLE) || isErrorState()}
                        onClick={(e) => {
                    step(state);
                }}>
                    Step
                </button>
                <button className="btn-operation bg-teal-600 hover:bg-teal-700"
                        disabled={state !== STATE.PAUSED}
                        onClick={(e) => {
                    // TODO step back
                }}>
                    Step Back
                </button>
                <button className="btn-operation bg-orange-500 hover:bg-orange-700" onClick={(e) => {
                    reset();
                }}>
                    Reset
                </button>
            </div>
            <div className="mt-2 mb-2 row">
                <textarea
                    disabled={state !== STATE.IDLE && state !== STATE.STOPPED}
                    onChange={(e) => setLines(e.currentTarget.value)}
                    placeholder="Enter some ezasm code..."
                />
            </div>
            <p className="mt-2 mb-2">{isErrorState() ? getErrorState() : result}</p>
        </div>
    );
}

export default App;
