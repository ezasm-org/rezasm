import React, {useCallback, useEffect, useReducer, useRef, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {CALLBACKS_TRIGGERS, CALLBACK_TYPES} from "../App.jsx";
import {RUST} from "../rust_functions.js";

const ENTER = 13;

function Console({loaded, registerCallback, exitCode, error}) {
    const input = useRef(null);
    const historyScrollbox = useRef(null);

    const history = useRef([]);
    const [inputText, setInputText] = useState("");

    // forceUpdate is a hack to allow history to be modified as a ref instantly but still
    // be able to rerender after the change is made
    const [, forceUpdate] = useReducer(() => Date.now());

    const setHistory = (newHistory) => {
        history.current = newHistory;
        forceUpdate();
    };

    const appendHistory = (lines) => {
        if (history.current.length === 0) {
            history.current = lines;
        } else {
            const first = lines.shift();
            history.current[history.current.length - 1] += first;
            history.current = [...history.current, ...lines];
        }
        historyScrollbox.current.scrollTop = historyScrollbox.current.scrollHeight;
        forceUpdate();
    };

    const reset = () => {
        setHistory([]);
        setInputText("");
    };

    const onInputChange = (event) => {
        setInputText(event.target.value);
    };

    const onKeyPress = useCallback((event) => {
        if (event.keyCode === ENTER) {
            appendHistory([inputText, ""]);
            setInputText("");
            RUST.RECEIVE_INPUT({data: inputText});
        }
    }, [inputText]);

    // new logic will be needed if this effect ever is called more than once to prevent multiple
    // listeners from being made. hopefully loaded never remounts.
    useEffect(() => {
        if (loaded && !window.__TAURI_IPC__) {
            console.log("worker registered");
            window.worker.on("wasm_print", (data) => {
                appendHistory(data.split("\n"));
            });
        }
    }, [loaded]);

    useEffect(() => {
        if (window.__TAURI_IPC__) {
            const unlisten = listen("tauri_print", (event) => {
                appendHistory(event.payload.split("\n"));
            });
            return () => unlisten.then(f => f());
        }
    }, []);

    useEffect(() => {
        if (exitCode !== "") {
            const toHistory = [`Program exited with exit code ${exitCode}`, ""];
            if (history.current.length > 0 && history.current[history.current.length - 1] !== "") {
                toHistory.unshift("");
            }
            appendHistory(toHistory);
        }
    }, [exitCode]);

    registerCallback(CALLBACKS_TRIGGERS.RESET, CALLBACK_TYPES.CONSOLE, reset);

    let consoleHistoryHtml;
    if (history.current.length === 0) {
        consoleHistoryHtml = <></>;
    } else if (history.current.length === 1) {
        consoleHistoryHtml = <>{history.current[0]}</>;
    } else {
        consoleHistoryHtml = <>{history.current.reduce((left, right) => <> {left} <br/> {right} </>)}</>;
    }

    return (
        <div className="console"
            onClick={() => input.current?.focus()}>
            <div className="console-history-scrollbox" ref={historyScrollbox}>
                <code className="console-history-text">
                    {consoleHistoryHtml}
                    {error ? <p className="console-error-text">{error}</p> : <></>}
                </code>
                <br/> {/* This is a temporary workaround to an issue where scrolling goes to the second to last element */}
            </div>
            <hr/>
            <code className="console-input-box row">
                <span className="console-prefix">&gt;&nbsp;</span>
                <input
                    className="console-input-text"
                    ref={input}
                    value={inputText}
                    disabled={error !== ""}
                    onChange={onInputChange}
                    onKeyDown={onKeyPress}
                />
            </code>
        </div>
    );
}

export default Console;
