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

//! Selection text, word picking, search and URLs.

#[path = "support/term.rs"]
mod support;

use nonos_vt::url::find_urls;
use nonos_vt::Pos;
use support::term;

fn p(line: u64, col: usize) -> Pos {
    Pos { line, col }
}

#[test]
fn a_wrapped_line_copies_as_one() {
    let t = term(4, 3, "abcdef\r\ngh");
    assert_eq!(t.text_between(p(0, 0), p(2, 3), false), "abcdef\ngh");
}

#[test]
fn block_selection_takes_columns() {
    let t = term(4, 2, "abcd\r\nefgh");
    assert_eq!(t.text_between(p(0, 1), p(1, 2), true), "bc\nfg");
}

#[test]
fn a_double_click_picks_a_path_whole() {
    let t = term(20, 1, "cat /etc/hosts now");
    let (a, b) = t.word_at(p(0, 7));
    assert_eq!((a.col, b.col), (4, 13));
}

#[test]
fn search_finds_across_a_wrap_and_wraps_around() {
    let t = term(4, 3, "xxhello");
    assert_eq!(t.find("hello", p(0, 0), false, true), Some((p(0, 2), p(1, 2))));
    assert_eq!(t.find("HELLO", p(1, 3), false, false), Some((p(0, 2), p(1, 2))));
    assert_eq!(t.find("nope", p(0, 0), false, true), None);
}

#[test]
fn urls_drop_trailing_punctuation() {
    let s: Vec<char> = "see https://a.io/x_(y). and http://b".chars().collect();
    let found: Vec<String> = find_urls(&s).iter().map(|&(a, b)| s[a..b].iter().collect()).collect();
    assert_eq!(found, ["https://a.io/x_(y)", "http://b"]);
}
