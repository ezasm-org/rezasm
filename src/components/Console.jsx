import {listen} from "@tauri-apps/api/event";
import {useCallback, useEffect, useRef, useState} from "react";
import {RESET_CALLBACKS} from "../App.jsx";

const ENTER = 13;

function Console({registerCallback}) {
    const terminal = useRef(null);
    const input = useRef(null);
    const [inputText, setInputText] = useState("");

    const history = useRef([]);

    const reset = useCallback(() => {
        history.current = [];
        setInputText("");
    }, []);

    registerCallback(RESET_CALLBACKS.CONSOLE, reset);

    const printString = useCallback((string) => {
        history.current = [...history.current, string];
    }, [history]);

    const onInputChange = useCallback((event) => {
        setInputText(event.target.value);
    }, []);

    const onKeyPress = useCallback((event) => {
        if (event.keyCode === ENTER) {
            const inputString = inputText + "\n";
            history.current = [...history.current, inputString];
            setInputText("");
            // TODO send the input to the rust
        }
    }, [history, inputText]);

    useEffect(() => {
        const unlisten = listen("tauri_print", (event) => {
            console.log(event.payload);
            printString(event.payload);
        });
        return () => unlisten.then(f => f());
    }, [printString]);

    return (
        <div className="terminal"
            ref={terminal}>
            <div className="terminal-history">
                <code>
                    {
                        history.current.reduce((left, right) => left + right, "")
                    }
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
