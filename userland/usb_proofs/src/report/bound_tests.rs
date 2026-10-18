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

//! The walk's bounds on sizes, each met and passed by one: report size,
//! report count, size times count, and the bits of one report.

use super::build::{app, input};
use crate::descriptors::report::parse;

#[test]
fn sizes_and_counts_are_bounded_one_by_one_and_together() {
    assert!(parse(&app(&[&input(256, 1)])).is_ok());
    assert!(parse(&app(&[&input(257, 1)])).is_err(), "a field of 257 bits");
    assert!(parse(&app(&[&input(1, 2048)])).is_ok());
    assert!(parse(&app(&[&input(1, 2049)])).is_err(), "2049 fields");
    assert!(parse(&app(&[&input(32, 64)])).is_ok());
    assert!(parse(&app(&[&input(32, 65)])).is_err(), "2080 bits in one item");
    let wide = input(256, 1);
    assert!(parse(&app(&[&wide[..]; 8])).is_ok());
    assert!(parse(&app(&[&wide[..]; 9])).is_err(), "2304 bits in one report");
    // Four-byte sizes and counts far past any bound.
    assert!(parse(&app(&[&[0x77, 0xff, 0xff, 0xff, 0xff]])).is_err());
    assert!(parse(&app(&[&[0x97, 0xff, 0xff, 0xff, 0xff]])).is_err());
    // An Output or Feature item is held to the same bound.
    assert!(parse(&app(&[&[0x75, 0x20, 0x95, 0x41, 0xb1, 0x02]])).is_err());
}
