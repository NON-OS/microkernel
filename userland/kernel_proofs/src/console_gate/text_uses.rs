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

//! Every file under the kernel's src/, and two readings of a file's code
//! that a plain search for a name gets wrong: a call of a function by its
//! own name, and a `use` that gives a function another one.

use super::text::{root, strip};

/// Every .rs file under src/, its comments taken out, as (path under src/,
/// code), in path order.
pub fn every_file() -> Vec<(String, String)> {
    let (src, mut found) = (root(), Vec::new());
    let mut dirs = vec![src.clone()];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).expect("src/ unreadable") {
            let path = entry.expect("a src/ entry").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read(&path).expect("a src/ file");
                let rel = path.strip_prefix(&src).expect("under src/");
                let code = strip(&String::from_utf8_lossy(&text));
                found.push((rel.to_string_lossy().into_owned(), code));
            }
        }
    }
    found.sort();
    found
}

fn ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// How many times `code` calls `name(`, not counting a longer name that
/// ends in it.
pub fn calls(code: &str, name: &str) -> usize {
    let call = format!("{name}(");
    let starts = code.match_indices(&call).map(|(i, _)| i);
    starts.filter(|&i| !code[..i].ends_with(ident)).count()
}

/// Every `a as b` inside a `use` in `code`, as (a, b).
pub fn renames(code: &str) -> Vec<(String, String)> {
    let code = code.split_whitespace().collect::<Vec<_>>().join(" ");
    let word = |s: &str| -> String { s.chars().take_while(|&c| ident(c)).collect() };
    let mut found = Vec::new();
    for (at, _) in code.match_indices(" as ") {
        let before = &code[..at];
        let item = before.rmatch_indices("use ").map(|(u, _)| u);
        let mut item = item.filter(|&u| !before[..u].ends_with(ident));
        if item.next().is_none_or(|u| before[u..].contains(';')) {
            continue;
        }
        let back: String = word(&before.chars().rev().collect::<String>());
        found.push((back.chars().rev().collect(), word(&code[at + 4..])));
    }
    found
}
