import {STATE} from "../App.jsx";

function Editor({state, setCode}) {
    return (
        <textarea
            disabled={state.current !== STATE.IDLE && state.current !== STATE.STOPPED}
            onChange={(e) => setCode(e.currentTarget.value)}
            placeholder="Enter some ezasm code..."
        />
    );
}

export default Editor;
