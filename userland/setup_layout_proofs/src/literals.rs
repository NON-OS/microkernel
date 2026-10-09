// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The text a screen draws, read out of its source. Setup's lines are byte
//! string literals and the installer's are string literals; this collects
//! every one in a capsule's source files, escapes resolved, so the fit proofs
//! measure each line that ships rather than a list someone keeps by hand.
//! Comments are skipped, and a literal holding a newline (a log line, never
//! drawn) is left out.

use std::fs;
use std::path::Path;

/// One literal: the file it is in, and its text.
pub struct Literal {
    pub file: String,
    pub text: String,
}

/// Every literal in the .rs files under `dir`, relative to this crate:
/// byte strings when `bytes`, else plain strings.
pub fn literals(dir: &str, bytes: bool) -> Vec<Literal> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
    let mut files = Vec::new();
    walk(&root, &mut files);
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let src = fs::read_to_string(&path).expect("readable source");
        let file = path.strip_prefix(&root).unwrap_or(&path).display().to_string();
        for text in scan(&src, bytes) {
            if !text.contains('\n') {
                out.push(Literal { file: file.clone(), text });
            }
        }
    }
    out
}

fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).expect("readable directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn scan(src: &str, bytes: bool) -> Vec<String> {
    let s: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        let next = s.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            while i < s.len() && s[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && next == Some('*') {
            i += 2;
            while i + 1 < s.len() && !(s[i] == '*' && s[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if c == '\'' {
            // A char literal ('"', '\'', 'x'); anything else is a lifetime.
            if next == Some('\\') {
                i += 2;
                while i < s.len() && s[i] != '\'' {
                    i += 1;
                }
                i += 1;
                continue;
            }
            if s.get(i + 2) == Some(&'\'') {
                i += 3;
                continue;
            }
            i += 1;
            continue;
        }
        let ident_before = i > 0 && (s[i - 1].is_alphanumeric() || s[i - 1] == '_');
        let byte_open = c == 'b' && next == Some('"') && !ident_before;
        if byte_open || c == '"' {
            let start = if byte_open { i + 2 } else { i + 1 };
            let (text, end) = read(&s, start);
            if byte_open == bytes {
                out.push(text);
            }
            i = end;
            continue;
        }
        i += 1;
    }
    out
}

/// The literal starting at `i`, just past its opening quote: its text and the
/// index past its closing quote.
fn read(s: &[char], mut i: usize) -> (String, usize) {
    let mut text = String::new();
    while i < s.len() && s[i] != '"' {
        if s[i] != '\\' {
            text.push(s[i]);
            i += 1;
            continue;
        }
        let e = s.get(i + 1).copied().unwrap_or('\\');
        i += 2;
        match e {
            'n' => text.push('\n'),
            't' => text.push('\t'),
            'r' => text.push('\r'),
            '0' => text.push('\0'),
            'x' => {
                let hex: String = s[i..i + 2].iter().collect();
                text.push(u8::from_str_radix(&hex, 16).expect("hex escape") as char);
                i += 2;
            }
            'u' => {
                let close = s[i..].iter().position(|&c| c == '}').expect("unicode escape");
                let hex: String = s[i + 1..i + close].iter().collect();
                let code = u32::from_str_radix(&hex, 16).expect("unicode escape");
                text.push(char::from_u32(code).expect("scalar value"));
                i += close + 1;
            }
            '\n' => {
                // A line continuation: the newline and the next line's
                // leading spaces are not part of the text.
                while i < s.len() && s[i].is_whitespace() {
                    i += 1;
                }
            }
            other => text.push(other),
        }
    }
    (text, i + 1)
}

#[test]
fn the_scanner_reads_escapes_and_skips_comments_and_chars() {
    let src = r#"
        // "not this"
        /* "nor this" */
        let a = b"Refused \"";
        let q = '"';
        let b = b"\xD8 and \\ end";
        let c = "plain";
        fn f<'a>(x: &'a [u8]) {}
        let d = b"tail";
    "#;
    let bytes = scan(src, true);
    assert_eq!(bytes, vec!["Refused \"", "\u{d8} and \\ end", "tail"]);
    assert_eq!(scan(src, false), vec!["plain"]);
}
