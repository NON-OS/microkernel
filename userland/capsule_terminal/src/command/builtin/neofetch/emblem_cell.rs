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

//! One cell of the emblem in 24-bit colour: two pixels, the upper as the
//! half block's colour and the lower as the cell's background.

use alloc::vec::Vec;

use crate::term::util::format_u64;

/// A colour as red, green and blue, each 0 to 255.
pub type Rgb = [u32; 3];

fn sgr(out: &mut Vec<u8>, lead: &[u8], c: Rgb) {
    out.extend_from_slice(lead);
    for v in c {
        let mut d = [0u8; 4];
        let n = format_u64(u64::from(v.min(255)), &mut d);
        out.push(b';');
        out.extend_from_slice(&d[..n]);
    }
}

/// Append the cell for pixels `up` and `down`; None is the background.
pub fn cell(out: &mut Vec<u8>, up: Option<Rgb>, down: Option<Rgb>) {
    let glyph = match (up, down) {
        (None, None) => {
            out.push(b' ');
            return;
        }
        (Some(top), bottom) => {
            out.extend_from_slice(b"\x1b[");
            sgr(out, b"38;2", top);
            if let Some(b) = bottom {
                sgr(out, b";48;2", b);
            }
            "▀"
        }
        (None, Some(bottom)) => {
            out.extend_from_slice(b"\x1b[");
            sgr(out, b"38;2", bottom);
            "▄"
        }
    };
    out.push(b'm');
    out.extend_from_slice(glyph.as_bytes());
    out.extend_from_slice(b"\x1b[0m");
}

/// The plain cell for a file: which halves the face fills.
pub fn block(up: bool, down: bool) -> &'static [u8] {
    match (up, down) {
        (true, true) => "█".as_bytes(),
        (true, false) => "▀".as_bytes(),
        (false, true) => "▄".as_bytes(),
        (false, false) => b" ",
    }
}
