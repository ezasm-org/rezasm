import React, {useEffect, useRef, useState} from "react";
import RegistryView from "./components/RegistryView.jsx";
import {loadWasm} from "./rust_functions.js";

import "../dist/output.css";
import MemoryView from "./components/MemoryView.jsx";
import Console from "./components/Console.jsx";
import Controls from "./components/Controls.jsx";
import Editor from "./components/Editor.jsx";
import {useSimulator} from "./components/simulator.js";

function App() {

    const {
        state,
        error,
        exitCode,
        setState,
        setCode,
        setInstructionDelay,
        registerCallback,
        start,
        stop,
        step,
        load,
        reset
    } = useSimulator();
    const [wasmLoaded, setWasmLoaded] = useState(false);

    useEffect(() => {
        loadWasm()
            .then((loaded) => setWasmLoaded(loaded))
            .catch(() => setWasmLoaded(false));
    }, []);

    return (
        <div className="container">
            <div className="fill">
                <Controls state={state} setState={setState} start={start} stop={stop} step={step} reset={reset} load={load} error={error}/>
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
