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

//! Turns the VIC data file into a table at compile time. A malformed line
//! stops the build: a shipping table is never half read.

use super::vic::Vic;
use super::vic_lex::{number, skip_line, spaces};

/// One line per VIC: `vic width height rate p|i clock_khz`, separated by
/// spaces; `#` starts a comment line and blank lines are skipped.
pub const fn parse(src: &[u8]) -> [Vic; 256] {
    let mut t = [Vic::NONE; 256];
    let mut at = 0;
    while at < src.len() {
        if src[at] == b'#' || src[at] == b'\n' {
            at = skip_line(src, at);
            continue;
        }
        let (code, a) = number(src, at);
        let (width, a) = number(src, a);
        let (height, a) = number(src, a);
        let (hz, a) = number(src, a);
        let a = spaces(src, a);
        assert!(src[a] == b'p' || src[a] == b'i', "scan must be p or i");
        let interlaced = src[a] == b'i';
        let (pixel_khz, a) = number(src, a + 1);
        assert!(a == src.len() || src[a] == b'\n', "trailing text after the clock");
        assert!(code > 0 && code < 256 && width > 0 && height > 0, "bad VIC line");
        assert!(t[code as usize].width == 0, "VIC listed twice");
        t[code as usize] = Vic {
            width: width as u16,
            height: height as u16,
            hz: hz as u16,
            interlaced,
            pixel_khz,
        };
        at = skip_line(src, a);
    }
    t
}
