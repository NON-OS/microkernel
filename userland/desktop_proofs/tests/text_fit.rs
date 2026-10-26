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
//! A line too long for its width is cut with an ellipsis on a character
//! boundary, and one that fits is left whole (toolkit text/fit.rs).

use nonos_toolkit::text::{fit_end, ELLIPSIS};

/// Ten units a character, the ellipsis included.
fn tens(s: &str) -> u32 {
    s.chars().count() as u32 * 10
}

#[test]
fn a_line_that_fits_is_whole() {
    assert_eq!(fit_end("Files", 50, tens), "Files");
    assert_eq!(fit_end("", 0, tens), "");
}

#[test]
fn a_long_line_is_cut_with_an_ellipsis_that_fits() {
    let cut = fit_end("Settings and privacy", 80, tens);
    assert!(cut.ends_with(ELLIPSIS) && tens(&cut) <= 80, "{cut}");
    assert_eq!(cut, format!("Setting{ELLIPSIS}"));
    assert_eq!(fit_end("Ab cd", 40, tens), format!("Ab{ELLIPSIS}"), "no space before it");
}

#[test]
fn a_cut_never_splits_a_character() {
    assert_eq!(fit_end("NØNOS Wallet", 40, tens), format!("NØN{ELLIPSIS}"));
    assert_eq!(fit_end("ØØØØØØ", 40, tens), format!("ØØØ{ELLIPSIS}"), "beside a two-byte one");
}

#[test]
fn too_narrow_for_the_ellipsis_is_nothing() {
    assert_eq!(fit_end("Files", 10, tens), ELLIPSIS);
    assert_eq!(fit_end("Files", 9, tens), "");
}
