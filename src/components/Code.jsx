import React, { useEffect, useState, useRef } from "react";
import RegistryView from "./RegistryView.jsx";
import { loadWasm } from "../rust_functions.ts";
import { Tabs, Tab } from "./Tabs.jsx";

import MemoryView from "./MemoryView.jsx";
import Console from "./Console.jsx";
import Controls from "./Controls.jsx";
import Editor from "./Editor.jsx";
import { useSimulator } from "./simulator.ts";
// import { saveFile, loadFile } from "./SaveLoad.jsx";


/*
    This is where the pieces of gui are initialized

    The Code component handles the actual usage of the gui buttons created and applies them to the Tauri app
*/

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

    // Create a ref to access the textarea in the Editor component
    const editorRef = useRef(null);

    useEffect(() => {
        loadWasm()
            .then((loaded) => setWasmLoaded(loaded))
            .catch(() => setWasmLoaded(false));
    }, []);

    // Save file functionality
    const saveFile = () => {
        const code = editorRef.current?.value; // Access the textarea value using the ref
        if (!code) {
            alert("No code to save!");
            return;
        }

        // Get the file name from the input field
        const fileNameInput = document.getElementById("file-name");
        const fileName = fileNameInput?.value || "code.txt"; // Default to "code.txt" if no name is provided

        const blob = new Blob([code], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = fileName; // Use the user-provided file name
        a.click();
        URL.revokeObjectURL(url);
    };

    // Load file functionality
    const loadFile = (event) => {
        const file = event.target.files[0];
        if (!file) {
            alert("No file selected!");
            return;
        }
        const reader = new FileReader();
        reader.onload = (e) => {
            const content = e.target.result;
            if (editorRef.current) {
                editorRef.current.value = content; // Set the textarea value using the ref
                setCode(content); // Update the code in the simulator state
            }
        };
        reader.readAsText(file);
    };

    return (
        <div>
            {/* File Dropdown and Name Input Container */}
            <div className="file-controls">
                <input
                    type="text"
                    id="file-name"
                    placeholder="Enter file name"
                    className="file-name-input"
                />
                <select
                    className="file-dropdown"
                    onChange={(e) => {
                        const selectedOption = e.target.value;

                        if (selectedOption === "save") {
                            saveFile(); // Trigger save functionality
                        } else if (selectedOption === "load") {
                            document.getElementById("file-input").click(); // Trigger file input for load
                        }

                        // Reset the dropdown to the default option
                        e.target.value = ""; // Set the value back to the default
                    }}
                >
                    <option value="" disabled selected>
                        File options
                    </option>
                    <option value="save">Save Code</option>
                    <option value="load">Load Code</option>
                </select>
                {/* Hidden file input for loading files */}
                <input
                    id="file-input"
                    type="file"
                    accept=".txt"
                    onChange={loadFile}
                    style={{ display: "none" }}
                />
            </div>

            {/* Controls Section */}
            <div className="top-bar">
                <Controls
                    state={state}
                    setState={setState}
                    start={start}
                    stop={stop}
                    step={step}
                    reset={reset}
                    load={load}
                    error={error}
                    stepBack={stepBack}
                />
            </div>
            
            
            <div className="fill px-4"> {/* initialize the editor */}
                <div className="mt-2 mb-2 row codearea">
                    <div className="w-5/6 h-full pe-4">
                        {/* Pass the ref to the Editor component */}
                        <Editor state={state} setCode={setCode} editorRef={editorRef} />
                    </div>
                    <div className="w-1/6">
                        <RegistryView
                            loaded={wasmLoaded}
                            registerCallback={registerCallback}
                        />
                    </div>
                </div>
            </div>
            <Tabs>
                <Tab label="Console">
                    <div className="fill" id="tabs_console" data-tab-active>
                        <Console
                            loaded={wasmLoaded}
                            registerCallback={registerCallback}
                            exitCode={exitCode}
                            error={error}
                        />
                    </div>
                </Tab>
                <Tab label="Memory Viewer">
                    <div className="fill" id="tabs_memory">
                        <MemoryView
                            loaded={wasmLoaded}
                            registerCallback={registerCallback}
                        />
                    </div>
                </Tab>
            </Tabs>
        </div>
    );
}

export default Code;
