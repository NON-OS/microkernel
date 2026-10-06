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

//! A page reached by a short .anyone name says so next to the address, in
//! net.anon's own words; a full onion style address and every other host
//! say nothing. The notice wraps at spaces to the band's width.

use capsule_browser_proofs::browser::short_name::{notice_for, wrap, SHORT_NOTICE};

const FULL: &str = "http://iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone/";

#[test]
fn a_short_name_carries_the_notice() {
    assert_eq!(notice_for("http://shield.anyone/"), Some(SHORT_NOTICE));
    assert_eq!(notice_for("https://shield.anyone/pay?x=1"), Some(SHORT_NOTICE));
}

#[test]
fn a_full_address_and_other_hosts_carry_none() {
    assert_eq!(notice_for(FULL), None);
    assert_eq!(notice_for("https://example.com/"), None);
    assert_eq!(notice_for("http://shield.example/"), None);
    assert_eq!(notice_for("not a url"), None);
}

/* Every character 7 pixels wide, so the cuts are known. */
fn mono(t: &str) -> i32 {
    t.len() as i32 * 7
}

#[test]
fn wrap_cuts_at_spaces_within_the_width() {
    assert_eq!(wrap("aa bb cc dd", 35, mono), vec!["aa bb", "cc dd"]);
    assert_eq!(wrap("aa bb cc dd", 1000, mono), vec!["aa bb cc dd"]);
}

#[test]
fn a_word_wider_than_the_line_stands_alone() {
    assert_eq!(wrap("a longword b", 35, mono), vec!["a", "longword", "b"]);
}

#[test]
fn the_notice_wraps_without_losing_a_word() {
    let lines = wrap(SHORT_NOTICE, 600, mono);
    assert!(lines.len() > 1);
    assert!(lines.iter().all(|l| mono(l) <= 600));
    assert_eq!(lines.join(" "), SHORT_NOTICE);
}
