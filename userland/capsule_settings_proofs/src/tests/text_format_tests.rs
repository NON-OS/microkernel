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

//! What the panel formats at paint time: a heading never cut inside a
//! character, numbers as digits, and the longest line fits its row.

use crate::text::valbuf::{ValBuf, CAP};

#[test]
fn a_query_written_into_a_full_heading_never_cuts_a_character() {
    let mut v = ValBuf::new();
    v.push_str(&"a".repeat(CAP - 1));
    v.push_str("é");
    assert_eq!(v.as_str(), "a".repeat(CAP - 1), "the two byte e does not fit in one");
    let mut v = ValBuf::new();
    v.push_str("No setting matches \"Größe\".");
    assert_eq!(v.as_str(), "No setting matches \"Größe\".");
}

/* The Wi-Fi rows format numbers and ids at paint time without allocating. */
#[test]
fn decimal_and_hex_go_in_as_digits() {
    let mut b = ValBuf::new();
    b.push_dec(0);
    b.push_str(" ");
    b.push_dec(4_294_967_295);
    b.push_str(" ");
    b.push_hex16(0x10EC);
    b.push_str(":");
    b.push_hex16(0xC821);
    assert_eq!(b.as_str(), "0 4294967295 10ec:c821");
}

#[test]
fn the_longest_no_driver_line_fits_the_row() {
    let mut b = ValBuf::new();
    b.push_str("Wi-Fi chip ");
    b.push_hex16(0x14C3);
    b.push_str(":");
    b.push_hex16(0x0616);
    b.push_str(" has no NONOS driver; use Ethernet or USB Wi-Fi");
    assert!(b.as_str().ends_with("USB Wi-Fi"), "{}", b.as_str());
    assert!(b.as_str().len() <= CAP);
}
