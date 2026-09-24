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
//! The two ends of the contrast range.

use crate::theme::contrast::{at_least, BODY, LARGE};
use crate::wcag;

/// Black on white is the maximum the formula can return. A check that could not
#[test]
fn black_on_white_is_the_maximum() {
    let ratio = wcag::ratio(0xFF00_0000, 0xFFFF_FFFF);
    assert!((ratio - 21.0).abs() < 0.01, "21:1, got {ratio:.3}");
    assert!(at_least(0xFF00_0000, 0xFFFF_FFFF, 210), "and the integer check sees it");
}
#[test]
fn a_colour_against_itself_passes_nothing() {
    for colour in [0xFF00_0000u32, 0xFFFF_FFFF, 0xFF35_C4E2, 0xFF2E_3440] {
        assert!((wcag::ratio(colour, colour) - 1.0).abs() < 1e-9);
        assert!(!at_least(colour, colour, LARGE));
        assert!(!at_least(colour, colour, BODY));
    }
}
