import {EditorView, basicSetup} from "codemirror"
import {EditorState} from "@codemirror/state"
import { useEffect, useRef } from "react"


export const CodeArea = ({onChange}) => {
    const editor = useRef()

    const onUpdate = EditorView.updateListener.of((v) =>
        onChange(v.state.doc.toString())
    )

    const startState = EditorState.create({
        extensions: [basicSetup,
                    onUpdate],
    })


    useEffect(() => {

        const view = new EditorView({
            state: startState,
            parent: editor.current,
        })

        return () => {
            view.destroy()
        }
    }, [])

    return <div ref={editor}/>
}
