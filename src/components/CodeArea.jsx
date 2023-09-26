import {EditorView, basicSetup} from "codemirror"
import {EditorState, Facet} from "@codemirror/state"
import { useEffect, useRef } from "react"
import STATE from "../App.jsx"


export const CodeArea = ({onChange, disableState}) => {
    const editor = useRef()

    const onUpdate = EditorView.updateListener.of((v) =>
        onChange(v.state.doc.toString())
    )

    const disabledWhen = EditorState.readOnly.of(() => disableState === 1)

    const startState = EditorState.create({
        extensions: [basicSetup,
                    onUpdate,
                    disabledWhen],
    })


    useEffect(() => {

        const view = new EditorView({
            state: startState,
            parent: editor.current
        })

        return () => {
            view.destroy()
        }
    }, [])

    return <div className="w-full" ref={editor}/>
}
