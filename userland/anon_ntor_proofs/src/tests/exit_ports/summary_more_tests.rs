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

//! The bound on what one line keeps, and the microdescriptor that carries
//! the line.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::directory::microdesc::policy::exits_to;
use crate::directory::microdesc::summary::{summary, RANGES_MAX};
use crate::directory::microdesc::{parse, Microdesc};

/// A line of `verb` over the odd ports 1, 3, 5 and on, `count` of them.
fn odd_ports(verb: &str, count: usize) -> String {
    let ports: Vec<String> = (0..count).map(|i| alloc::format!("{}", 2 * i + 1)).collect();
    alloc::format!("{verb} {}", ports.join(","))
}

#[test]
fn a_line_at_the_bound_is_kept_whole_and_one_past_it_is_cut() {
    let at = summary(odd_ports("accept", RANGES_MAX).as_bytes()).unwrap();
    assert_eq!((at.ranges.len(), at.cut), (RANGES_MAX, false));
    let past = summary(odd_ports("accept", RANGES_MAX + 1).as_bytes()).unwrap();
    assert_eq!((past.ranges.len(), past.cut), (RANGES_MAX, true), "bounded, and marked");
    let last_kept = (2 * RANGES_MAX - 1) as u16;
    assert!(exits_to(&past, last_kept), "a port it kept is still let out");
    assert!(!exits_to(&past, last_kept + 2), "the dropped port is not");
}

#[test]
fn a_reject_line_past_the_bound_lets_nothing_out() {
    let past = summary(odd_ports("reject", RANGES_MAX + 1).as_bytes()).unwrap();
    assert!(past.cut);
    for port in [2u16, 4, 1000, 18081, 65535] {
        assert!(!exits_to(&past, port), "{port} may be in the part dropped");
    }
    let at = summary(odd_ports("reject", RANGES_MAX).as_bytes()).unwrap();
    assert!(exits_to(&at, 2) && exits_to(&at, 18081) && !exits_to(&at, 1));
}

#[test]
fn a_microdescriptor_keeps_the_line_beside_its_web_answer() {
    let id = b"ntor-onion-key 4k42YwphMtr96Q1qlyhy6O9RB6h9xzkmtxEMqd3mU3I\n\
               id ed25519 lzrs8z5HT9mg13o5JX1V3cED9ZSk8HbMUMtxrbrY3OM\n";
    let with = |p: &[u8]| parse(&[&id[..], p].concat()).expect("parses");
    let m = with(b"p accept 80,443,18081\n");
    assert!(m.exits_web);
    assert_eq!(m.ports.ranges, [(80, 80), (443, 443), (18081, 18081)]);
    let bad = with(b"p accept 80,443,x\n");
    assert!(bad.exits_web, "the web answer is read exactly as before");
    assert_eq!(bad.ports, Microdesc::default().ports, "the malformed line lets nothing else out");
    assert!(!exits_to(&with(b"").ports, 18081), "no line at all exits nowhere");
}
