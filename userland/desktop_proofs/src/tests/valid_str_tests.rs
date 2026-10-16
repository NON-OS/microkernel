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

//! The shell draws a toast, a tile name or a consent line from bytes another
//! capsule handed it. Malformed bytes draw the valid part, never nothing.

use crate::ui_font::valid_str;

#[test]
fn valid_text_is_drawn_whole() {
    assert_eq!(valid_str(b"Wallet"), "Wallet");
    assert_eq!(valid_str("N\u{d8}NOS".as_bytes()), "N\u{d8}NOS");
    assert_eq!(valid_str(b""), "");
}

#[test]
fn malformed_text_is_drawn_up_to_the_first_bad_byte() {
    assert_eq!(valid_str(b"Clock\xffbroken"), "Clock");
    assert_eq!(valid_str(b"\xc3"), "");
    // A name cut inside a two-byte character keeps what came before it.
    let cut = &"Caf\u{e9}".as_bytes()[..4];
    assert_eq!(valid_str(cut), "Caf");
}
