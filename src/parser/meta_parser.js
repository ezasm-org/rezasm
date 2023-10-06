import {parser} from "./parser.js"
import { foldNodeProp, foldInside, indentNodeProp } from "@codemirror/language"
import { styleTags, tags as t } from "@lezer/highlight"

let meta_parser = parser.configure(
    {
        props: [
            styleTags({
            })
        ]
    }
)
