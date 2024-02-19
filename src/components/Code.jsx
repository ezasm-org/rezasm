import React, {useEffect, useState} from "react";
import RegistryView from "./RegistryView.jsx";
import {loadWasm} from "../rust_functions.js";
import Tabs from ".Tabs.jsx";

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
        load,
        reset,
        history,
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
                    <div className="w-3/4 h-full pe-4">
                        <Editor state={state} setCode={setCode} />
                    </div>
                    <div className="w-1/4">
                        <RegistryView loaded={wasmLoaded} registerCallback={registerCallback} />
                    </div>
                </div>
            </div>
            <Tabs>
            </Tabs>
            <div className="fill hidden" id="tabs_console" data-tab-active>
                <Console loaded={wasmLoaded} registerCallback={registerCallback} exitCode={exitCode} error={error} history={history}/>
            </div>
            <div className="fill" id="tabs_memory">
                <MemoryView loaded={wasmLoaded} registerCallback={registerCallback} />
            </div>
        </div>
    );
}

export default Code;
