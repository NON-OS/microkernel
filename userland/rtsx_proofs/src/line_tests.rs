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

//! The log line a photo shows.

use crate::line::{Line, LINE_BYTES};

#[test]
fn a_line_is_tagged_and_numbers_read_plainly() {
    let mut line = Line::start();
    line.text(b"SD card ").dec(62177280).text(b" rca ").hex(0xB368, 4);
    assert_eq!(line.finish(), b"rtsx: SD card 62177280 rca b368\n");
    let mut zero = Line::start();
    assert_eq!(zero.dec(0).finish(), b"rtsx: 0\n");
}

#[test]
fn a_long_line_is_cut_and_still_ends_in_a_newline() {
    let mut line = Line::start();
    for _ in 0..40 {
        line.text(b"abcdef");
    }
    let out = line.finish();
    assert_eq!(out.len(), LINE_BYTES);
    assert_eq!(out[LINE_BYTES - 1], b'\n');
}
