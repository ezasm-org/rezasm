import {listen} from "@tauri-apps/api/event";
import {useCallback, useEffect, useReducer, useRef, useState} from "react";
import {CALLBACKS_TRIGGERS, CALLBACK_TYPES} from "../App.jsx";

const ENTER = 13;

function Console({registerCallback, exitCode}) {
    const terminal = useRef(null);
    const input = useRef(null);

    const history = useRef([]);
    const [inputText, setInputText] = useState("");

    const [, forceUpdate] = useReducer(() => Date.now());

    const setHistory = useCallback((newHistory) => {
        history.current = newHistory;
        forceUpdate();
    }, []);

    const appendHistory = useCallback((lines) => {
        if (history.current.length === 0) {
            history.current = lines;
        }
        else {
            const first = lines.shift();
            history.current[history.current.length-1] += first;
            history.current = [...history.current, ...lines];
        }
        forceUpdate();
    }, []);

    const reset = useCallback(() => {
        setHistory([]);
        setInputText("");
    }, []);

    registerCallback(CALLBACKS_TRIGGERS.RESET, CALLBACK_TYPES.CONSOLE, reset);

    const onInputChange = useCallback((event) => {
        setInputText(event.target.value);
    }, []);

    const onKeyPress = useCallback((event) => {
        if (event.keyCode === ENTER) {
            const inputString = inputText + "\n";
            setHistory([...history.current, inputString]);
            setInputText("");
            forceUpdate();
            // TODO send the input to the rust
        }
    }, [history, inputText]);

    useEffect(() => {
        const unlisten = listen("tauri_print", (event) => {
            const lines = event.payload.split("\n");
            appendHistory(lines);
            forceUpdate();
        });
        return () => unlisten.then(f => f());
    }, [history]);

    useEffect(() => {
        if (exitCode !== "") {
            appendHistory(
                (((history.current.length > 0) ? "\n" : "") + `Program exited with exit code ${exitCode}\n`).split("\n")
            );

        }
    }, [exitCode]);

    return (
        <div className="terminal"
            ref={terminal}>
            <div className="terminal-history">
                <code>
                    {history.current.reduce((left, right) => <> {left} <br/> {right} </>, <></>)}
                </code>
            </div>
            <div>
                <input
                    className="terminal-input-line"
                    type="text"
                    ref={input}
                    value={inputText}
                    onChange={onInputChange}
                    onKeyDown={onKeyPress}
                />
            </div>
        </div>
    );
}

export default Console;
