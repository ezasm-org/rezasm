import React from "react";
import {STATE} from "./simulator.ts";

function Editor({state, setCode}) {
    return (
        <textarea
            disabled={state.current !== STATE.IDLE && state.current !== STATE.STOPPED}
            onChange={(e) => setCode(e.currentTarget.value)}
            placeholder="Enter some ezasm code..."
            className="codearea"
        />
    );
}

export default Editor;
