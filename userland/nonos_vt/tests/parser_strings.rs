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

//! OSC, DCS and APC strings, oversized input, and UTF-8.

#[path = "support/log.rs"]
mod log;
use log::{run, Log};
use nonos_vt::parser::Parser;

#[test]
fn osc_ends_on_bel_or_st() {
    assert_eq!(run(b"\x1b]0;hi\x07x"), ["osc0;hi", "px"]);
    assert_eq!(run(b"\x1b]2;yo\x1b\\x"), ["osc2;yo", "esc\\", "px"]);
}

#[test]
fn dcs_payload_and_apc_swallowed() {
    assert_eq!(run(b"\x1bP+q544e\x1b\\"), ["dcs+q544e", "esc\\"]);
    assert_eq!(run(b"\x1b_Gsecret\x1b\\z"), ["esc\\", "pz"]);
}

#[test]
fn oversized_osc_is_dropped_whole() {
    let mut s = b"\x1b]0;".to_vec();
    s.extend(std::iter::repeat_n(b'a', 10_000));
    s.extend_from_slice(b"\x07z");
    assert_eq!(run(&s), ["pz"]);
}

#[test]
fn utf8_split_across_feeds() {
    let mut p = Parser::new();
    let mut l = Log::default();
    p.feed(&mut l, "é".as_bytes().split_at(1).0);
    p.feed(&mut l, "é".as_bytes().split_at(1).1);
    assert_eq!(l.0, ["pé"]);
}

#[test]
fn utf8_cut_by_a_control_shows_a_replacement() {
    assert_eq!(run(b"\xc3\n"), ["p\u{fffd}", "x0a"]);
    assert_eq!(run(b"\xc0\xaf"), ["p\u{fffd}", "p\u{fffd}"]);
    assert_eq!(run(b"\xed\xa0\x80"), ["p\u{fffd}"]);
}
