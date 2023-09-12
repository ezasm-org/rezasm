import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { wasm_completion_callback, wasm_load, wasm_run, wasm_step, wasm_stop, wasm_reset, wasm_is_completed,
         wasm_get_exit_status, wasm_get_register_value } from "../dist/wasm";
import init from "../dist/wasm/rezasm_wasm.js"
import "../dist/output.css";
import _ from "lodash";

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
        return new Promise((resolve, reject) => {
            try {
                wasm_load(lines);
                resolve(null);
            } catch (error) {
                reject(error);
            }
        });
    } else {
        throw "Function load does not exist";
    }
}

const rust_run = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_run", {});
    } else if (wasm_run) {
        return new Promise((resolve, reject) => {
            wasm_run();
            resolve(null);
        });
    } else {
        throw "Function run does not exist";
    }
}

const rust_step = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_step", {});
    } else if (wasm_step) {
        return new Promise((resolve, reject) => {
            wasm_step();
            resolve(null);
        });
    } else {
        throw "Function step does not exist";
    }
}

const rust_reset = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_reset", {});
    } else if (wasm_reset) {
        return new Promise((resolve, reject) => {
            wasm_reset();
            resolve(null);
        });
    } else {
        throw "Function reset does not exist";
    }
}

const rust_stop = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_stop", {});
    } else if (wasm_stop) {
        return new Promise((resolve, reject) => {
            wasm_stop();
            resolve(null);
        });
    } else {
        throw "Function stop does not exist";
    }
}

const rust_is_completed = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_is_completed", {});
    } else if (wasm_is_completed) {
        return new Promise((resolve, reject) => resolve(wasm_is_completed()));
    } else {
        throw "Function is_completed does not exist";
    }
}

const rust_get_exit_status = async () => {
    if (window.__TAURI__) {
        return invoke("tauri_get_exit_status", {});
    } else if (wasm_get_exit_status) {
        return new Promise((resolve, reject) => resolve(wasm_get_exit_status()));
    } else {
        throw "Function get_exit_status does not exist";
    }
}

const rust_get_register_value = async register => {
    if (window.__TAURI__) {
        return invoke("tauri_get_register_value", {register});
    } else if (wasm_get_register_value) {
        return new Promise((resolve, reject) => resolve(wasm_get_register_value(register)));
    } else {
        throw "Function get_register_value does not exist";
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

    const debounce = useCallback(_.debounce((func, arg) => func(arg), 250,
        {leading: true, trailing: false, maxWait: 250}), []);

    const isCompleted = useCallback(async () => {
        return await rust_is_completed();
    }, []);

    const getExitStatus = useCallback(async () => {
        return await rust_get_exit_status();
    }, []);

    const getRegisterValue = useCallback(async register => {
        return await rust_get_register_value(register);
    }, []);

    const reset = useCallback(async () => {
        await rust_reset();
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
            })
            .catch(error => {
                setErrorState(error);
                currentState = STATE.STOPPED;
            });

        return currentState;
    }, [setErrorState, lines]);

    const reset_load = useCallback(async () => {
        return reset().then(newState => {
            return load(newState);
        });
    }, [reset, load]);

    const run = useCallback(async currentState => {
        if (currentState === STATE.RUNNING) {
            return currentState;
        }

        reset_load().then(async newState => {
            if (newState >= STATE.LOADED && !isErrorState()) {
                if (await isCompleted()) {
                    setResult("Program exited with exit code " + await getExitStatus());
                    setState(currentState);
                } else {
                    setState(STATE.RUNNING);
                    rust_run();
                }
            }
        });

        setState(currentState);
        return currentState;
    }, [reset, load, getExitStatus, isCompleted]);

    const step = useCallback(async currentState => {
        if (currentState < STATE.LOADED) {
            reset_load().then(async () => {
                if (await isCompleted()) {
                    currentState = STATE.STOPPED;
                    setState(currentState);
                    setResult("Program exited with exit code " +  await getExitStatus());
                } else {
                    currentState = STATE.PAUSED;
                    setState(currentState);
                    rust_step();
                }
            })
        } else if (currentState === STATE.PAUSED) {
            if (await isCompleted()) {
                currentState = STATE.STOPPED;
                setState(currentState);
                setResult("Program exited with exit code " +  await getExitStatus());
            } else {
                currentState = STATE.PAUSED;
                setState(currentState);
                rust_step();
            }
        }

        return currentState
    }, [load, isErrorState, getExitStatus, isCompleted]);

    const stop = useCallback(async currentState => {
        await rust_stop();
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
            init().then(() => {
                if (wasm_completion_callback) {
                    window.wasm_completion_callback = wasm_completion_callback;
                }
            });
        }
    }, [])

    return (
        <div className="container">
            <h1><b>Welcome to rezasm!</b></h1>
            <div className="mt-2 mb-2 row">
                    { state === STATE.RUNNING ?
                    <button className="btn-operation bg-red-500 hover:bg-red-700" disabled={state !== STATE.RUNNING || isErrorState()} onClick={(e) => {
                        debounce(stop, state);
                    }}>
                        Stop
                    </button>
                    :
                    <button className="btn-operation bg-green-500 hover:bg-green-700" disabled={state !== STATE.IDLE || isErrorState()} onClick={(e) => {
                        debounce(run, state);
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
                            debounce(step, state);
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
                    debounce(reset, state);
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
