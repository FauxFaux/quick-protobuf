use std::process::id;
use convert_case::{Case, Casing};
use crate::types::Config;

static RUST_KEYWORDS: [&str; 75] = [
    "abstract",
    "alignof",
    "as",
    "become",
    "bool",
    "box",
    "Box",
    "break",
    "BytesReader",
    "const",
    "continue",
    "crate",
    "Cow",
    "Default",
    "do",
    "else",
    "enum",
    "Err",
    "extern",
    "f32",
    "f64",
    "false",
    "final",
    "fn",
    "for",
    "HashMap",
    "i32",
    "i64",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "None",
    "MessageWrite",
    "offsetof",
    "Ok",
    "Option",
    "override",
    "priv",
    "pub",
    "pure",
    "ref",
    "Result",
    "return",
    "self",
    "Self",
    "sizeof",
    "Some",
    "static",
    "str",
    "String",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "typeof",
    "u8",
    "u32",
    "u64",
    "unsafe",
    "unsized",
    "use",
    "Vec",
    "virtual",
    "where",
    "while",
    "Write",
    "Writer",
    "yield",
];

pub fn sanitize_pascal(ident: &mut String, config: &Config) {
    *ident = map_dotted(ident, |part| sanitize_part(&part.to_case(Case::Pascal)))
}

pub fn sanitize_snake(ident: &mut String, config: &Config) {
    *ident = map_dotted(ident, |part| sanitize_part(&part.to_case(Case::Snake)))
}

fn map_dotted(ident: &str, func: impl FnMut(&str) -> String) -> String {
    ident.split('.').map(func).collect::<Vec<_>>().join(".")
}

/// Check if the identifier is a Rust keyword and appends a `_pb` suffix if that's the case
///
/// @deprecated Use sanitize_camel or sanitize_pascal instead.
#[deprecated]
pub fn sanitize_keyword(ident: &mut String) {
    *ident = map_dotted(ident, sanitize_part);
}

pub fn sanitize_part(ident: &str) -> String {
    if RUST_KEYWORDS.contains(&ident) {
        format!("{}_pb", ident)
    } else {
        ident.to_string()
    }
}
