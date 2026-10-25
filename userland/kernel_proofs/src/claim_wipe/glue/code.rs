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

//! Kernel source as the glue checks compare it: every comment taken out, a
//! line comment and a block comment alike, nested ones too, so a call left
//! only in a comment never passes for code; string and character literals
//! are kept whole, so a `//` inside one is not taken for a comment. A raw
//! string is read as a plain one: none of the files the checks read has one.

/// `src` with each comment made one space, its layout otherwise kept.
pub(super) fn code(src: &str) -> String {
    let (c, mut out, mut i) = (src.chars().collect::<Vec<_>>(), String::new(), 0);
    while i < c.len() {
        let gone = match (c[i], c.get(i + 1)) {
            ('/', Some('/')) => c[i..].iter().position(|&x| x == '\n').unwrap_or(c.len() - i),
            ('/', Some('*')) => block(&c[i..]),
            _ => 0,
        };
        let kept = if gone == 0 { literal(&c, i).min(c.len() - i) } else { 0 };
        out.extend(if gone == 0 { &c[i..i + kept] } else { &[' '][..] });
        i += gone + kept;
    }
    out
}

/// `code`, each run of whitespace made one space.
pub(super) fn flat(src: &str) -> String {
    code(src).split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The length of the block comment `c` begins with, nested ones inside it.
fn block(c: &[char]) -> usize {
    let (mut depth, mut k) = (0u32, 0);
    while k + 1 < c.len() {
        match (c[k], c[k + 1]) {
            ('/', '*') => (depth, k) = (depth + 1, k + 2),
            ('*', '/') => (depth, k) = (depth - 1, k + 2),
            _ => k += 1,
        }
        if depth == 0 {
            return k;
        }
    }
    c.len()
}

/// The length of the string or character literal `c[i]` begins, or 1.
fn literal(c: &[char], i: usize) -> usize {
    let end = match (c[i], c.get(i + 1), c.get(i + 2)) {
        ('"', ..) => {
            let mut k = i + 1;
            while k < c.len() && c[k] != '"' {
                k += 1 + usize::from(c[k] == '\\');
            }
            k
        }
        ('\'', Some('\\'), _) => (i + 3..c.len()).find(|&k| c[k] == '\'').unwrap_or(i),
        ('\'', _, Some('\'')) => i + 2,
        _ => i,
    };
    end + 1 - i
}
