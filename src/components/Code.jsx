import React, {useEffect, useState} from "react";
import RegistryView from "./RegistryView.jsx";
import {loadWasm} from "../rust_functions.js";

import MemoryView from "./MemoryView.jsx";
import Console from "./Console.jsx";
import Controls from "./Controls.jsx";
import Editor from "./Editor.jsx";
import {useSimulator} from "./simulator.js";

function Code() {

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
        step_back,
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
                <Controls state={state} setState={setState} start={start} stop={stop} step={step} step_back={step_back} reset={reset} load={load} error={error}/>
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

export default Code;
