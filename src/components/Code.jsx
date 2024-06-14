import React, {useEffect, useState} from "react";
import RegistryView from "./RegistryView.jsx";
import {loadWasm} from "../rust_functions.ts";
import {Tabs, Tab} from "./Tabs.jsx";

import MemoryView from "./MemoryView.jsx";
import Console from "./Console.jsx";
import Controls from "./Controls.jsx";
import Editor from "./Editor.jsx";
import {useSimulator} from "./simulator.ts";

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
        stepBack,
        load,
        reset,
    } = useSimulator();
    const [wasmLoaded, setWasmLoaded] = useState(false);

    useEffect(() => {
        loadWasm()
            .then((loaded) => setWasmLoaded(loaded))
            .catch(() => setWasmLoaded(false));
    }, []);

    return (
        <div>
            <div className="fill px-4">
                <Controls state={state} setState={setState} start={start} stop={stop} step={step} reset={reset} load={load} error={error} stepBack={stepBack}/>
                <div className="mt-2 mb-2 row codearea">
                    <div className="w-5/6 h-full pe-4">
                        <Editor state={state} setCode={setCode} />
                    </div>
                    <div className="w-1/6">
                        <RegistryView loaded={wasmLoaded} registerCallback={registerCallback} />
                    </div>
                </div>
            </div>
            <Tabs>
                <Tab label="Console">
                    <div className="fill" id="tabs_console" data-tab-active>
                        <Console loaded={wasmLoaded} registerCallback={registerCallback} exitCode={exitCode} error={error} />
                    </div>
                </Tab>
                <Tab label="Memory Viewer">
                    <div className="fill" id="tabs_memory">
                        <MemoryView loaded={wasmLoaded} registerCallback={registerCallback} />
                    </div>
                </Tab>
            </Tabs>
        </div>
    );
}

export default Code;
