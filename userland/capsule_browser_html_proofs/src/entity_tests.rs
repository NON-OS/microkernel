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

//! Character references: the tokenizer's decoder (13.2.5.72 to 13.2.5.80)
//! over the full named table, and the lenient `push_decoded` beside it.

use crate::browser::html::entity::charref;

pub fn cref(rest: &str, in_attr: bool) -> (String, usize) {
    let mut out = String::new();
    let used = charref(rest, in_attr, &mut out);
    (out, used)
}

#[test]
fn the_longest_name_in_the_table_wins() {
    assert_eq!(cref("notin;x", false), ("\u{2209}".into(), 6));
    assert_eq!(cref("notit;", false), ("\u{AC}".into(), 3));
    assert_eq!(cref("ampx", false), ("&".into(), 3));
    assert_eq!(cref("zwnj;", false), ("\u{200C}".into(), 5));
    assert_eq!(cref("AElig;", false), ("\u{C6}".into(), 6));
    let long = "CounterClockwiseContourIntegral;";
    assert_eq!(cref(long, false), ("\u{2233}".into(), long.len()));
    assert_eq!(cref("NotNestedGreaterGreater;", false), ("\u{2AA2}\u{338}".into(), 24));
    assert_eq!(cref("zzzz;", false), ("&".into(), 0));
    assert_eq!(cref(" x", false), ("&".into(), 0));
}

#[test]
fn numeric_references_follow_the_replacement_rules() {
    assert_eq!(cref("#65;", false), ("A".into(), 4));
    assert_eq!(cref("#x41", false), ("A".into(), 4));
    assert_eq!(cref("#x80;", false), ("\u{20AC}".into(), 5));
    assert_eq!(cref("#0;", false), ("\u{FFFD}".into(), 3));
    assert_eq!(cref("#xD800;", false), ("\u{FFFD}".into(), 7));
    assert_eq!(cref("#x110000;", false), ("\u{FFFD}".into(), 9));
    assert_eq!(cref("#99999999999999999999;", false), ("\u{FFFD}".into(), 22));
    assert_eq!(cref("#;", false), ("&#".into(), 1));
    assert_eq!(cref("#xg;", false), ("&#x".into(), 2));
}
