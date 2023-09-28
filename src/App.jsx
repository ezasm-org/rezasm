import React, {useRef} from "react";
import {useCallback, useEffect, useState} from "react";
import _ from "lodash";
import RegisterView from "./components/RegisterView.jsx";
import WorkerPromise from "webworker-promise";
import init from "../dist/wasm/rezasm_wasm.js";
import {
    rust_get_exit_status,
    rust_get_register_names,
    rust_get_register_value,
    rust_get_register_values,
    rust_is_completed,
    rust_load,
    rust_reset,
    rust_step,
    rust_stop
} from "./rust_functions.js";

import "../dist/output.css";

const STATE = {
    IDLE: 1,
    LOADING: 2,
    LOADED: 3,
    RUNNING: 4,
    PAUSED: 5,
    STOPPED: 1,
};

function App() {
    const [lines, setLines] = useState("");
    const [error, setError] = useState("");
    const [result, setResult] = useState("");
    const [state, setState] = useState(STATE.IDLE);
    const timerId = useRef(null);
    const [instructionDelay, setInstructionDelay] = useState(5);

    const [registers, setRegisters] = useState([]);
    const [registerNames, setRegisterNames] = useState([]);

    const disallowExecution = () => {
        if (timerId.current !== null) {
            clearTimeout(timerId.current);
            timerId.current = null;
        }
    };

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

    const stop = useCallback(async currentState => {
        disallowExecution();
        await rust_stop();
        currentState = STATE.STOPPED;
        setState(currentState);
        return currentState;
    }, []);

    const reset = useCallback(async () => {
        disallowExecution();
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
                setState(STATE.STOPPED);
            });

        setState(currentState);
        return currentState;
    }, [lines, setErrorState]);

    const reset_load = useCallback(async () => {
        return reset().then(newState => {
            setState(STATE.LOADING);
            return load(newState);
        });
    }, [load, reset]);

    const checkAndHandleProgramCompletion = useCallback(async () => {
        setRegisters(await rust_get_register_values());
        if (await isCompleted() || isErrorState()) {
            disallowExecution();
            setState(STATE.STOPPED);
            setResult("Program exited with exit code " +  await getExitStatus());
            return true;
        } else {
            return false;
        }
    }, [getExitStatus, isCompleted, isErrorState]);

    const handleStepCall = useCallback(async () => {
        rust_step()
            .then(async () => await checkAndHandleProgramCompletion())
            .catch(error => {
                setErrorState(error);
                setState(STATE.STOPPED);
            });
    }, [checkAndHandleProgramCompletion, setErrorState]);

    const step = useCallback(async currentState => {
        if (currentState < STATE.LOADED) {
            reset_load().then(async newState => {
                if (newState !== STATE.STOPPED && ! await checkAndHandleProgramCompletion()) {
                    return handleStepCall().then(() => setState(STATE.PAUSED));
                }
            });
        } else if (currentState === STATE.PAUSED || currentState === STATE.RUNNING) {
            return handleStepCall();
        }
    }, [reset_load, checkAndHandleProgramCompletion, handleStepCall]);

    const recursivelyCallStep = useCallback(async () => {
        if (state > STATE.RUNNING) {
            return;
        }
        checkAndHandleProgramCompletion().then(async completed => {
            if (!completed) {
                handleStepCall().then(() => {
                    timerId.current = setTimeout(recursivelyCallStep, instructionDelay);
                }).catch((e) => {
                    timerId.current = null;
                    setErrorState(e);
                    setState(STATE.STOPPED);
                });
            }
        });
    }, [state, checkAndHandleProgramCompletion, handleStepCall, instructionDelay, setErrorState]);

    const run = useCallback(async () => {
        reset_load().then(async newState => {
            if (newState !== STATE.STOPPED) {
                setState(STATE.RUNNING);
                recursivelyCallStep();
            }
        });
    }, [reset_load, recursivelyCallStep]);

    useEffect(() => {
        if (init) {
            window.worker = new WorkerPromise(new Worker("/src/worker.js", { type: "module" }));
            init().then(() => {
                window.worker.postMessage({command: "ping"}).then(e => {
                    if (e !== "pong") {
                        throw Error("Could not communicate with the web-worker");
                    }
                    rust_get_register_names().then(result => setRegisterNames(result));
                    rust_get_register_values().then(result => setRegisters(result));
                });
            });
        }

    }, []);

    return (
        <div className="container">
            <div>
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
                <div className="mt-2 mb-2 row codearea">
                    <textarea
                        disabled={state !== STATE.IDLE && state !== STATE.STOPPED}
                        onChange={(e) => setLines(e.currentTarget.value)}
                        placeholder="Enter some ezasm code..."
                    />
                    <RegisterView registerNames={registerNames} registers={registers} />
                </div>
                <p className="mt-2 mb-2">{isErrorState() ? getErrorState() : result}</p>
            </div>
        </div>
    );
}

export default App;
