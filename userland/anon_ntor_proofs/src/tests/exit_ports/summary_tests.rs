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

//! The `p` line kept whole: the verb, the ranges and the single ports, read
//! as Tor's parse_short_policy reads them.

use crate::directory::microdesc::summary::summary;

#[test]
fn an_accept_line_keeps_its_ports_and_ranges_in_order() {
    let s = summary(b"accept 20-23,43,53,79-81,443,18080-18089").expect("well formed");
    assert!(!s.reject);
    assert_eq!(s.ranges, [(20, 23), (43, 43), (53, 53), (79, 81), (443, 443), (18080, 18089)]);
    assert!(!s.cut);
}

#[test]
fn a_reject_line_keeps_its_verb() {
    let s = summary(b"reject 25,119,135-139,445,563,1214,4661-4666").expect("well formed");
    assert!(s.reject, "the ranges are what is refused");
    assert_eq!(s.ranges[0], (25, 25));
    assert_eq!(s.ranges[2], (135, 139));
    assert_eq!(s.ranges.len(), 7);
}

#[test]
fn the_extreme_ports_and_a_one_port_range_are_well_formed() {
    assert_eq!(summary(b"accept 1-65535").unwrap().ranges, [(1, 65535)]);
    assert_eq!(summary(b"accept 1,65535").unwrap().ranges, [(1, 1), (65535, 65535)]);
    assert_eq!(summary(b"reject 7-7").unwrap().ranges, [(7, 7)]);
}

#[test]
fn a_malformed_line_is_refused_whole() {
    for bad in [
        &b""[..],
        b"accept",
        b"accept ",
        b"allow 80",
        b"Accept 80",
        b"accept 80,",
        b"accept ,80",
        b"accept 80,,443",
        b"accept 0",
        b"accept 0-80",
        b"accept 65536",
        b"accept 1-65536",
        b"accept 443-80",
        b"accept -80",
        b"accept 80-",
        b"accept 1-2-3",
        b"accept 8o",
        b"accept +80",
        b"accept 80 443",
        b"accept 99999999999999999999999",
    ] {
        assert!(summary(bad).is_none(), "{:?}", core::str::from_utf8(bad));
    }
}
