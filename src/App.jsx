import {useCallback, useEffect, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../dist/output.css";

const STATE = {
    IDLE: 1,
    LOADED: 2,
    RUNNING: 3,
    PAUSED: 4,
    STOPPED: 5,
}

const RESULT_OK = "data";
const RESULT_ERR = "error";

const isOk = result => {
    return result[RESULT_OK] || result[RESULT_OK] === null;
}

const isError = result => {
    return result[RESULT_ERR] || result[RESULT_ERR] === null;
}

const getOk = result => {
    if (isOk(result)) {
        return result[RESULT_OK] === null ? {} : result[RESULT_OK];
    } else {
        return undefined;
    }
}

const getErr = result => {
    if (isError(result)) {
        return result[RESULT_ERR] === null ? {} : result[RESULT_ERR];
    } else {
        return undefined;
    }
}

const isSome = option => {
    return option !== null;
}

function App() {
    const [lines, setLines] = useState("");
    const [error, setError] = useState("");
    const [result, setResult] = useState("");

    const [state, setState] = useState(STATE.IDLE);
    const [counter, setCounter] = useState(0);

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
        return await invoke("is_completed", {});
    }, []);

    const getExitStatus = useCallback(async () => {
        return await invoke("get_exit_status", {});
    }, []);

    const getRegisterValue = useCallback(async register => {
        return await invoke("get_register_value", {register});
    }, []);

    const reset = useCallback(async () => {
        await invoke("reset", {});
        setState(STATE.IDLE);
        setResult("");
        clearErrorState();
        return STATE.IDLE;
    }, [clearErrorState]);

    const load = useCallback(async (currentState) => {
        if (currentState >= STATE.LOADED) {
            return currentState;
        }
        let result = await invoke("load", {lines});
        if (isOk(result)) {
            setState(STATE.LOADED);
            return STATE.LOADED;
        } else {
            setErrorState(getErr(result));
            setState(STATE.STOPPED);
            return STATE.STOPPED;
        }
    }, [setErrorState, isErrorState, lines, state]);

    const run = useCallback(async currentState => {
        currentState = await reset();
        currentState = await load(currentState);

        if (currentState >= STATE.LOADED) {
            currentState = STATE.RUNNING;
            invoke("run", {}).then(async () => {
                if (await isCompleted()) {
                    setResult("Program exited with exit code " +  await getExitStatus());
                    currentState = STATE.STOPPED;
                }
            });
        }
        setState(currentState);
        return currentState;
    }, [reset, load, error, isErrorState, getExitStatus, state]);

    const step = useCallback(async currentState => {
        if (currentState < STATE.LOADED) {
            currentState = await load(currentState);
            currentState = STATE.PAUSED;
        }

        if (currentState >= STATE.LOADED && currentState !== STATE.STOPPED) {
            currentState = STATE.PAUSED;
            invoke("step", {}).then(async () => {
                if (await isCompleted()) {
                    setResult("Program exited with exit code " +  await getExitStatus());
                    currentState = STATE.STOPPED;
                }
            });
        }
        setState(currentState);
        return currentState
    }, [load, isErrorState, getExitStatus, isCompleted, state]);

    const stop = useCallback(async currentState => {
        await invoke("stop", {})
        currentState = STATE.STOPPED;
        setState(currentState);
        return currentState;
    }, []);

    useEffect(() => {
        if (state === STATE.RUNNING || state === STATE.PAUSED) {
            isCompleted().then(completed => {
                if (completed) {
                    getExitStatus().then(exitStatus => {
                        setResult("Program exited with exit code " +  exitStatus);
                    });
                    setState(STATE.STOPPED);
                } else {
                    // trigger rerender
                    setTimeout(() => setCounter(counter + 1), 50);
                }
            });
        }
    }, [counter, state]);

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
