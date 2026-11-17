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

//! `wrap_hard`: every word back in order, each line within the width unless one
//! char alone is wider, a word cut only at its last char that fits, else `wrap`.

use nonos_toolkit::widgets::text::{width, wrap, wrap_hard, wrap_hard_by};

use super::samples::{sizes, tens, RUNS};
use super::serial::serial;

/// What is wrong with `lines` as `wrap_hard(s, max_w, px)`, if anything.
fn fault(s: &str, max_w: u32, px: f32, lines: &[(usize, usize)]) -> Option<String> {
    let (mut seen, mut kept) = (0, String::new());
    for &(a, b) in lines {
        if a < seen || a >= b || !s.is_char_boundary(a) || !s.is_char_boundary(b) {
            return Some(format!("({a}, {b}) is out of order or off a char boundary"));
        }
        let (gap, line) = (&s[seen..a], &s[a..b]);
        if !gap.chars().all(char::is_whitespace) {
            return Some(format!("{gap:?} is skipped"));
        }
        let next = s[b..].chars().next().filter(|c| !c.is_whitespace());
        let more = next.map(|c| width(&format!("{line}{c}"), px) <= max_w);
        if width(line, px) > max_w && line.chars().count() > 1 || more == Some(true) {
            return Some(format!("{line:?} is over {max_w}, or a cut that stops short"));
        }
        (seen, kept) = (b, kept + line);
    }
    let all: String = s.split_whitespace().collect();
    (kept.split_whitespace().collect::<String>() != all).then(|| format!("{kept:?} is not {s:?}"))
}

#[test]
fn wrap_hard_gives_back_the_text_in_lines_that_fit() {
    let _g = serial();
    let mut fails = Vec::new();
    for px in sizes() {
        for s in RUNS.into_iter().chain(["Grüße—日本語—ohne—Leerzeichen, 🙂🙂🙂🙂🙂"]) {
            for max_w in (0..=width(s, px) + 8).step_by(3) {
                let lines = wrap_hard(s, max_w, px);
                let long = s.split_whitespace().any(|w| width(w, px) > max_w);
                let odd = (!long && lines != wrap(s, max_w, px)).then(|| "not wrap's".into());
                let why = fault(s, max_w, px, &lines).or(odd);
                fails.extend(why.map(|w| format!("{s:?} at {px} in {max_w}: {w}")));
            }
        }
    }
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}

#[test]
fn wrap_hard_by_cuts_a_long_word_at_the_last_char_that_fits() {
    let s = "ab cdefghij k";
    assert_eq!(wrap_hard_by(s, 30, tens), [(0, 2), (3, 6), (6, 9), (9, 11), (12, 13)]);
    assert_eq!(wrap_hard_by(s, 40, tens), [(0, 2), (3, 7), (7, 11), (12, 13)]);
    assert_eq!(wrap_hard_by("abcdefg h", 50, tens), vec![(0, 5), (5, 9)]);
    assert_eq!(wrap_hard_by("日本語です", 20, tens), vec![(0, 6), (6, 12), (12, 15)]);
    assert_eq!(wrap_hard_by("üü", 5, tens), vec![(0, 2), (2, 4)]);
    assert_eq!(wrap_hard_by("ab", -1, tens), vec![(0, 1), (1, 2)]);
    assert_eq!(wrap_hard_by("  \t ", 50, tens), vec![]);
}
