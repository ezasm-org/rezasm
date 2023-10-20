import { foldNodeProp, foldInside, indentNodeProp, LRLanguage, LanguageSupport } from "@codemirror/language"
import { styleTags, tags as t } from "@lezer/highlight"
import { parser } from "./test_grammar"

export const
  LineComment = 1,
  Program = 2,
  Identifier = 3,
  String = 4,
  Boolean = 5

meta_parser = parser.configure(
    {
props: [
    styleTags({
      Identifier: t.variableName,
      Boolean: t.bool,
      String: t.string,
      LineComment: t.lineComment,
      "( )": t.paren
    }),
    indentNodeProp.add({
      Application: context => context.column(context.node.from) + context.unit
    }),
    foldNodeProp.add({
      Application: foldInside
    })
  ]
    }
)

const ez_language = LRLanguage.define({
    parser: meta_parser,
})

export function ez_language_support() {
    return new LanguageSupport(ez_language)
}
