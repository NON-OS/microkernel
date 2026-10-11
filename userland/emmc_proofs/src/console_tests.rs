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

//! The console line formatting.

use crate::emmc::text::Line;

#[test]
fn console_lines() {
    let mut l = Line::new();
    l.s(b"x ")
        .hex(0)
        .s(b" ")
        .hex(0x31cc)
        .s(b" ")
        .dec(0)
        .s(b" ")
        .dec(18_446_744_073_709_551_615)
        .s(b" ")
        .ascii(b"ab\x01c\0d");
    assert_eq!(l.as_bytes(), b"driver.emmc: x 0x0 0x31cc 0 18446744073709551615 ab?c");
    let mut long = Line::new();
    for _ in 0..100 {
        long.s(b"abc");
    }
    assert_eq!(long.as_bytes().len(), crate::emmc::text::LINE_MAX);
}
