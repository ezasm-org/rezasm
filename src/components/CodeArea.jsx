import CodeMirror from "@uiw/react-codemirror";
import { useCallback, useState } from "react";
import { createTheme } from "@uiw/codemirror-themes";
import { ez_language_support } from "../parser/meta_parser";

const myTheme = createTheme({
    theme: "light",
    settings: {
        background: "#ffffff",
        backgroundImage: "",
        foreground: "#75baff",
        caret: "#5d00ff",
        selection: "#036dd626",
        selectionMatch: "#036dd626",
        lineHighlight: "#8a91991a",
        gutterBackground: "#fff",
        gutterForeground: "#8a919966",
    },
});

function CodeArea({onChange, isEditable}) {
    const [code, setCode] = useState("");
    const changeCallback = useCallback((val, viewUpdate) => {
        setCode(val);
        onChange(val);
    }, []);
    return (
        <div className="editor_container">
            <CodeMirror value={code}
                minHeight="50vh"
                onChange={changeCallback}
                editable={isEditable()}
                theme={myTheme}
                indentWithTab="true"
                extensions={ez_language_support}
            />
        </div>
    );
}

export default CodeArea;
