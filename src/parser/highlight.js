import {styleTags, tags as t} from "@lezer/highlight";

export const ezasmHighlighting = styleTags({
    Comment: t.lineComment,
    Character: t.character,
    StringLiteral: t.string,
    InstructionName: t.keyword,
    NumberLiteral: t.number,
    Label: t.variableName,
    Dereference: t.derefOperator,
    Register: t.constant,
    LabelReference: t.name,

    Identifier: t.variableName,
    Boolean: t.bool,
    String: t.string,
    LineComment: t.lineComment,
    "( )": t.paren
});

console.log(ezasmHighlighting);
