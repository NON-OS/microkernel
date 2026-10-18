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

//! A pasted Shield address, and the tail of an address the field shows.

use super::shield_fixtures::PINNED;
use crate::shield_typed::{pasted_address, tail_start};

#[test]
fn a_pasted_address_loses_its_scheme_and_its_spaces() {
    assert_eq!(pasted_address(PINNED), Some(PINNED));
    let schemed = alloc::format!("nox:{PINNED}");
    assert_eq!(pasted_address(&schemed), Some(PINNED));
    let linked = alloc::format!("  NOX://{PINNED}\n");
    assert_eq!(pasted_address(&linked), Some(PINNED));
    assert_eq!(pasted_address("12.5"), None);
    assert_eq!(pasted_address("nox:"), None);
    assert_eq!(pasted_address(""), None);
}

#[test]
fn the_tail_shown_is_the_longest_that_fits() {
    let ten = |t: &str| 10 * t.chars().count() as i32;
    assert_eq!(tail_start("abc", 95, ten), 0);
    assert_eq!(tail_start("", 0, ten), 0);
    let text = "abcdefghijklmnop";
    let at = tail_start(text, 95, ten);
    assert_eq!(&text[at..], "hijklmnop");
    /* Every start is a character boundary. */
    let wide = "\u{e9}\u{e9}\u{e9}\u{e9}";
    let at = tail_start(wide, 25, ten);
    assert_eq!(&wide[at..], "\u{e9}\u{e9}");
    /* The address the field holds most: about a dozen measures, not 2,009. */
    let asked = core::cell::Cell::new(0u32);
    let at = tail_start(PINNED, 400, |t| {
        asked.set(asked.get() + 1);
        ten(t)
    });
    assert_eq!(PINNED.len() - at, 40);
    assert!(asked.get() <= 13, "{} measures", asked.get());
}
