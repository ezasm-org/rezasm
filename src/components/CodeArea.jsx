import CodeMirror from '@uiw/react-codemirror';
import { useCallback, useState } from 'react';
import { createTheme } from '@uiw/codemirror-themes';

const myTheme = createTheme({
  theme: 'light',
  settings: {
    background: '#ffffff',
    backgroundImage: '',
    foreground: '#75baff',
    caret: '#5d00ff',
    selection: '#036dd626',
    selectionMatch: '#036dd626',
    lineHighlight: '#8a91991a',
    gutterBackground: '#fff',
    gutterForeground: '#8a919966',
  },
});

function CodeArea({onChange, isEditable}) {
    const [code, setCode] = useState("");
    const changeCallback = useCallback((val, viewUpdate) => {
        setCode(val);
        onChange(val);
    }, []);
    return (
        <CodeMirror value={code} 
                    heght="200px" 
                    onChange={changeCallback} 
                    editable={isEditable()}
                    theme={myTheme}
                    indentWithTab="true"
                    />
    );
}

export default CodeArea;
