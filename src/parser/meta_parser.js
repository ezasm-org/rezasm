import { foldNodeProp, foldInside, indentNodeProp, LRLanguage, LanguageSupport } from "@codemirror/language";
import { styleTags, tags as t } from "@lezer/highlight";
import { parser } from "./parser.js";
import {ezasmHighlighting} from "./highlight.js";

export const
    LineComment = 1,
    Program = 2,
    Identifier = 3,
    String = 4,
    Boolean = 5;

const meta_parser = parser.configure(
    {
        props: [
            ezasmHighlighting,
            indentNodeProp.add({
                Application: context => context.column(context.node.from) + context.unit
            }),
            foldNodeProp.add({
                Application: foldInside
            })
        ]
    }
);

const ez_language = LRLanguage.define({
    parser: meta_parser,
    languageData: {
        commentTokens: {line: "#"}
    }
});

export function ez_language_support() {
    return new LanguageSupport(ez_language, []);
}
