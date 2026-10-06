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

//! Proofs for the language a path is shown as, and which files are coloured
//! as code: prose is left as typed.

use crate::highlight::{classify, Tok};
use crate::language::{language_name, tokens};

#[test]
fn the_status_bar_names_the_language_from_the_extension() {
    assert_eq!(language_name("/src/main.rs"), "Rust");
    assert_eq!(language_name("include/a.hpp"), "C++");
    assert_eq!(language_name("run.zsh"), "Shell");
    assert_eq!(language_name("notes.markdown"), "Markdown");
    assert_eq!(language_name("archive.tar.yml"), "YAML", "the last extension decides");
    assert_eq!(language_name("README"), "Text");
    assert_eq!(language_name(""), "Text");
}

#[test]
fn prose_is_not_coloured_as_code() {
    let line = b"fn from \"x\" // 42";
    for path in [&b"notes.txt"[..], b"README.md", b"LICENSE", &[0xff, 0xfe]] {
        let got = tokens(path, line);
        assert_eq!(got.len(), line.len());
        assert!(got.iter().all(|t| *t == Tok::Text));
    }
}

#[test]
fn code_is_coloured_as_the_highlighter_classifies_it() {
    let line = b"fn main() { let x = 42; } // done";
    let got = tokens(b"/src/main.rs", line);
    assert!(got == classify(line));
    assert!(got[0] == Tok::Keyword, "`fn` is a keyword");
    assert!(got.contains(&Tok::Comment));
}
