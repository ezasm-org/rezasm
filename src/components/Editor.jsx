import React, { useState, useEffect, useRef } from "react";
import { STATE } from "./simulator.ts";

function Editor({ state, setCode, editorRef }) {
    const [lineNumbers, setLineNumbers] = useState(["1"]);

    const updateLineNumbers = (e) => {
        const textarea = e.target;

        // Dynamically adjust the height of the textarea
        textarea.style.height = "auto"; // Reset height to calculate the new height
        textarea.style.height = `${textarea.scrollHeight}px`; // Set height to match content

        // Update line numbers
        const lines = textarea.value.split("\n").length;
        const lineArray = Array.from({ length: lines }, (_, i) => (i + 1).toString());
        setLineNumbers(lineArray);

        // Update the code in the parent component
        setCode(textarea.value);
    };

    const handleKeyDown = (e) => {
        if (e.key === "Tab") {
            e.preventDefault(); // Prevent the default tab behavior (moving focus)

            const textarea = e.target;
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;

            // Insert a tab character at the cursor position
            const value = textarea.value;
            textarea.value = value.substring(0, start) + "\t" + value.substring(end);

            // Move the cursor after the inserted tab
            textarea.selectionStart = textarea.selectionEnd = start + 1;

            // Trigger the change event to update the line numbers and parent state
            updateLineNumbers({ target: textarea });
        }
    };

    useEffect(() => {
        // Initialize line numbers when the component mounts
        const initialLines = editorRef.current?.value.split("\n").length || 1;
        setLineNumbers(Array.from({ length: initialLines }, (_, i) => (i + 1).toString()));
    }, []);

    return (
        <div className="editor-container">
            {/* Scrollable Container */}
            <div className="editor-scrollable">
                {/* Line Numbers */}
                <div className="line-numbers">
                    {lineNumbers.map((line) => (
                        <div key={line}>{line}</div>
                    ))}
                </div>

                {/* Text Area */}
                <textarea
                    ref={editorRef} // Attach the ref to the textarea
                    disabled={state.current !== STATE.IDLE && state.current !== STATE.STOPPED}
                    onChange={updateLineNumbers}
                    onKeyDown={handleKeyDown} // Handle the Tab key
                    placeholder="Enter some ezasm code..."
                    className="editor-textarea"
                />
            </div>
        </div>
    );
}

export default Editor;