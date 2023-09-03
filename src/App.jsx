import {useCallback, useEffect, useRef, useState} from "react";
import reactLogo from "./assets/react.svg";
import ezasmLogo from "./assets/ezasm.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "../dist/output.css";

const RESULT_OK = "data";
const RESULT_ERR = "error";

const isOk = (result) => {
    return result[RESULT_OK] || result[RESULT_OK] === null;
}

const isError = (result) => {
    return result[RESULT_ERR] || result[RESULT_ERR] === null;
}

const getOk = (result) => {
    if (isOk(result)) {
        return result[RESULT_OK] === null ? {} : result[RESULT_OK];
    } else {
        return undefined;
    }
}

const getErr = (result) => {
    if (isError(result)) {
        return result[RESULT_ERR] === null ? {} : result[RESULT_ERR];
    } else {
        return undefined;
    }
}

const isSome = (option) => {
    return option !== null;
}


function App() {
    const [error, setError] = useState("");
    const [result, setResult] = useState("");
    const [lines, setLines] = useState("");
    const loaded = useRef(false);
    const running = useRef(false);

    const isErrorState = useCallback(() => {
        return error !== "";
    }, [error]);

    const clearErrorState = useCallback(() => {
        setError("");
    }, []);

    const setErrorState = useCallback((newState) => {
        setError(newState);
        running.current = false;
    }, []);

    const getErrorState = useCallback(() => {
        return error;
    }, [error]);

    const reset = useCallback(async () => {
        await invoke("reset", {});
        loaded.current = false;
        clearErrorState();
        setResult("");
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
            console.log("Error in LOAD");
            // isErr
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

    const getRegisterValue = useCallback(async (register) => {
        return await invoke("get_register_value", {register});
    }, []);

    const run = useCallback(async () => {
        await reset();
        await load();

        if (loaded.current) {
            // TODO disable run button
            running.current = true;
            await invoke("run", {}).then(await (async (runResult) => {
                // TODO enable run button
                if (isOk(runResult)) {
                    setResult("Program exited with exit code " +  await getExitStatus());
                } else {
                    setErrorState(getErr(runResult));
                }
                running.current = false;
            }));
        }
    }, [reset, load, error, isErrorState, getExitStatus]);

    const step = useCallback(async () => {
        if (!loaded.current) {
            await load();
            if (loaded.current) {
                running.current = true;
            }
        }

        if (loaded.current) {
            // TODO Disable step button
            await invoke("step", {}).then(await (async (stepResult) => {
                // TODO Enable step button
                running.current = await isCompleted();
            }));
        }
    }, [load, isErrorState, isCompleted]);

    return (
        <div className="container">
            <h1>Welcome to rezasm!</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo"/>
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
                <a href="" target="_blank">
                    <img src={ezasmLogo} className="logo react" alt="EzASM logo"></img>
                </a>
            </div>
            <div className="row">
                <textarea
                    id="run-input"
                    cols="32"
                    onChange={(e) => setLines(e.currentTarget.value)}
                    placeholder="Enter some ezasm code..."
                />
                <button className="rounded-full bg-green-700" onClick={(e) => {
                    e.preventDefault();
                    run();
                }}>
                    Run
                </button>
                <button className="rounded-full bg-red-700" onClick={(e) => {
                    e.preventDefault();
                    step();
                }}>
                    Step
                </button>
                <button className="rounded-full bg-amber-800" onClick={(e) => {
                    e.preventDefault();
                    reset();
                }}>
                    Reset
                </button>
            </div>


            <p>{isErrorState() ? getErrorState() : result}</p>
        </div>
    );
}

export default App;
