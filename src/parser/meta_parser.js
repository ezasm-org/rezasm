import {parser} from "./parser.js"
import { foldNodeProp, foldInside, indentNodeProp, LRLanguage, LanguageSupport } from "@codemirror/language"
import { styleTags, tags as t } from "@lezer/highlight"

export const
  Space = 1,
  Comment = 2,
  Program = 3,
  Label = 4,
  Instruction = 5,
  InstructionName = 6,
  Terminal = 7,
  Character = 8,
  StringLiteral = 9,
  NumberLiteral = 10,
  Dereference = 11,
  Register = 12,
  LabelReference = 13

meta_parser = parser.configure(
    {
        props: [
            styleTags({
                Comment: t.comment,
                Label: t.variableName,
                InstructionName: t.function,
                StringLiteral: t.string,
                Character: t.character,
                NumberLiteral: t.number,
                Dereference: t.number,
                Register: t.variableName,
            }),
        ]
    }
)

const ez_language = LRLanguage.define({
    parser: meta_parser,
})

export function ez_language_support() {
    return new LanguageSupport(ez_language)
}
