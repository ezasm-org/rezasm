import {useCallback, useRef, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../dist/output.css";

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

    const loaded = useRef(false);
    const [running, setRunning] = useState(false);
    const [paused, setPaused] = useState(false);

    const isErrorState = useCallback(() => {
        return error !== "";
    }, [error]);

    const clearErrorState = useCallback(() => {
        setError("");
    }, []);

    const setErrorState = useCallback(newState => {
        setError(newState);
        setRunning(false);
        setPaused(false);
        loaded.current = false;
    }, []);

    const getErrorState = useCallback(() => {
        return error;
    }, [error]);

    const reset = useCallback(async () => {
        await invoke("reset", {});
        loaded.current = false;
        setRunning(false);
        setPaused(false);
        setResult("");
        clearErrorState();
    }, [clearErrorState]);

    const load = useCallback(async () => {
        if (loaded.current) {
            return true;
        }
        let result = await invoke("load", {lines});
        if (isOk(result)) {
            loaded.current = true;
            return true;
        } else {
            setErrorState(getErr(result));
            loaded.current = false;
            return false;
        }
    }, [setErrorState, isErrorState, lines]);

    const isCompleted = useCallback(async () => {
        return await invoke("is_completed", {});
    }, []);

    const getExitStatus = useCallback(async () => {
        return await invoke("get_exit_status", {});
    }, []);

    const getRegisterValue = useCallback(async register => {
        return await invoke("get_register_value", {register});
    }, []);

    const run = useCallback(async () => {
        await reset();
        await load();

        if (loaded.current) {
            // TODO disable run button
            setRunning(true);
            await invoke("run", {}).then(await (async runResult => {
                // TODO enable run button
                if (isOk(runResult)) {
                    setResult("Program exited with exit code " +  await getExitStatus());
                } else {
                    setErrorState(getErr(runResult));
                }
                setRunning(false);
            }));
        }
    }, [reset, load, error, isErrorState, getExitStatus]);

    const step = useCallback(async () => {
        if (!loaded.current) {
            if (await load()) {
                console.log("Loaded...");
                setPaused(true);
            }
        }

        if (loaded.current && !await isCompleted()) {
            // TODO Disable step button
            await invoke("step", {}).then(await (async stepResult => {
                // TODO Enable step button
                if (isOk(stepResult)) {
                    const completed = await isCompleted();
                    if (completed) {
                        setResult("Program exited with exit code " +  await getExitStatus());
                    }
                } else {
                    setErrorState(getErr(stepResult));
                }
            }));
        }
    }, [load, isErrorState, getExitStatus, isCompleted]);



    return (
        <div className="container">
            <h1><b>Welcome to rezasm!</b></h1>
            <div className="mt-2 mb-2 row">
                { running ?
                    <button className="btn-operation bg-red-500 hover:bg-red-700" onClick={(e) => {
                        // TODO stop
                    }}>
                        Stop
                    </button>
                    :
                    <button className="btn-operation bg-green-500 hover:bg-green-700" onClick={(e) => {
                        run();
                    }}>
                        Start
                    </button>
                }

                { paused ?
                    <button className="btn-operation bg-emerald-600 hover:bg-emerald-700" onClick={(e) => {
                        // TODO resume
                    }}>
                        Resume
                    </button>
                    :
                    <button className="btn-operation bg-cyan-600 hover:bg-cyan-700" onClick={(e) => {
                        // TODO pause
                    }}>
                        Pause
                    </button>
                }

                <button className="btn-operation bg-blue-500 hover:bg-blue-700" onClick={(e) => {
                    step();
                }}>
                    Step
                </button>
                <button className="btn-operation bg-teal-600 hover:bg-teal-700" onClick={(e) => {
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
                    disabled={loaded.current}
                    onChange={(e) => setLines(e.currentTarget.value)}
                    placeholder="Enter some ezasm code..."
                />
            </div>
            <p className="mt-2 mb-2">{isErrorState() ? getErrorState() : result}</p>
        </div>
    );
}

export default App;
