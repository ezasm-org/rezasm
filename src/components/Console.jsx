import {listen} from "@tauri-apps/api/event";
import {useCallback, useEffect, useReducer, useRef, useState} from "react";
import {CALLBACKS_TRIGGERS, CALLBACK_TYPES} from "../App.jsx";
import {rust_receive_input} from "../rust_functions.js";

const ENTER = 13;

function Console({registerCallback, exitCode, error}) {
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
        } else {
            const first = lines.shift();
            history.current[history.current.length - 1] += first;
            history.current = [...history.current, ...lines];
        }
        forceUpdate();
    }, []);

    const reset = useCallback(() => {
        setHistory([]);
        setInputText("");
    }, [setHistory]);

    registerCallback(CALLBACKS_TRIGGERS.RESET, CALLBACK_TYPES.CONSOLE, reset);

    const onInputChange = useCallback((event) => {
        setInputText(event.target.value);
    }, []);

    const onKeyPress = useCallback((event) => {
        if (event.keyCode === ENTER) {
            appendHistory([inputText, ""]);
            setInputText("");
            rust_receive_input(inputText); // send the input to rust
        }
    }, [appendHistory, inputText]);

    useEffect(() => {
        const unlisten = listen("tauri_print", (event) => {
            const lines = event.payload.split("\n");
            appendHistory([...lines]);
        });
        return () => unlisten.then(f => f());
    }, [appendHistory, history]);

    useEffect(() => {
        if (exitCode !== "") {
            const toHistory = [`Program exited with exit code ${exitCode}`, ""];
            if (history.current.length > 0 && history.current[history.current.length - 1] !== "") {
                toHistory.unshift("");
            }
            appendHistory(toHistory);
        }
    }, [appendHistory, exitCode]);

    let consoleHistoryHtml;
    if (history.current.length === 0) {
        consoleHistoryHtml = <></>;
    } else if (history.current.length === 1) {
        consoleHistoryHtml = <>{history.current[0]}</>
    } else {
        consoleHistoryHtml = <>{history.current.reduce((left, right) => <> {left} <br/> {right} </>)}</>;
    }

    return (
        <div className="console "
            ref={terminal}
            onClick={() => input.current?.focus()}>
            <div className="console-history-scrollbox">
                <code className="console-history-text">
                    {consoleHistoryHtml}
                    {error ? <p className="console-error-text">{error}</p> : <></>}
                </code>
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
