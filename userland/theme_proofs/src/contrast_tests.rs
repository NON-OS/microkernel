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
//! The WCAG transfer function, in integers.

use crate::theme::contrast::{at_least, BODY, LARGE};
use crate::theme::linear::LINEAR;
use crate::wcag;

#[test]
fn the_table_is_the_wcag_transfer_function() {
    for c in 0..=255u8 {
        let expected = {
            let x = c as f64 / 255.0;
            let l = if x <= 0.03928 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) };
            (l * 65535.0).round() as i64
        };
        let got = LINEAR[c as usize] as i64;
        assert_eq!(got, expected.min(65535), "linear[{c}] must be the function at {c}");
    }
}
#[test]
fn the_integer_check_agrees_with_the_formula() {
    let grey = |v: u8| 0xFF00_0000 | (v as u32) << 16 | (v as u32) << 8 | v as u32;
    let mut disagreed = 0;
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            let (x, y) = (grey(a), grey(b));
            let exact = wcag::ratio(x, y);
            for (floor, spec) in [(BODY, wcag::BODY), (LARGE, wcag::LARGE)] {
                if at_least(x, y, floor) == (exact >= spec) {
                    continue;
                }
                disagreed += 1;
                assert!(
                    (exact - spec).abs() < 0.001,
                    "grey {a} on grey {b} is {exact:.5}:1 and the integer check \
                     disagrees about a {spec} floor, which is not a tie"
                );
            }
        }
    }
    /*
     * Eight, measured, out of 130560 comparisons, and every one of them sat inside
     * the thousandth the assertion above allows. Pinned rather than left open: a
     * change to the arithmetic that started disagreeing anywhere else would show up
     * here as a count rather than slipping through as a tie.
     */
    assert_eq!(disagreed, 8, "the only disagreements are the known ties");
}
