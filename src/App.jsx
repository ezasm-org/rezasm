import { useState } from "react";
import reactLogo from "./assets/react.svg";
import ezasmLogo from "./assets/ezasm.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const RESULT_OK = "data";
const RESULT_ERR = "error";

const isOk = (result) => {
    // Cast to bool using ! operator, then undo inversion
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


function App() {
    const [error, setError] = useState("");
    const [loaded, setLoaded] = useState(false);
    const [running, setRunning] = useState(false);
    const [result, setResult] = useState("");
    const [lines, setLines] = useState("");

    const isErrorState = () => {
        return error !== "";
    }

    const clearErrorState = () => {
        setError("");
    }

    const setErrorState = (newState) => {
        setError(newState);
        setRunning(false);
    }

    const getErrorState = () => {
        return error;
    }

    async function reset() {
        await invoke("reset", {});
        setLoaded(false);
        clearErrorState();
        setResult("");
    }

    async function load() {
        let result = await invoke("load", {lines});
        if (isOk(result)) {
            setLoaded(true);
        } else {
            // isErr
            setLoaded(false);
            setErrorState(getErr(result));
        }
        return result;
    }

    async function isCompleted() {
        return await invoke("is_completed", {});
    }

    async function getExitStatus() {
        return await invoke("get_exit_status", {});
    }

    async function run() {
        await reset();
        await load();
        if (!isErrorState()) {
            // TODO disable run button
            setRunning(true);
            await invoke("run", {lines}).then(await (async (runResult) => {
                // TODO enable run button
                console.log("Async!!!!");
                if (isOk(runResult)) {
                    setResult("Program exited with exit code " +  await getExitStatus());
                } else {
                    setErrorState(getErr(runResult));
                }
                setRunning(false);
            }));
        }
    }

    async function step() {
        if (!loaded) {
            await load();
            if (!isErrorState()) {
                setRunning(true);
            }
        }

        if (loaded && running && !isErrorState()) {
            // TODO Disable step button
            await invoke("step", {}).then(await (async (stepResult) => {
                // TODO Enable step button
                setRunning(await isCompleted());
            }));
        }


    }

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
                <button onClick={(e) => {
                    e.preventDefault();
                    run();
                }}>
                    Run
                </button>
                <button onClick={(e) => {
                    e.preventDefault();
                    step();
                }}>
                    Step
                </button>
                <button onClick={(e) => {
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
