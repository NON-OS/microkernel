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

//! Pasted text to bytes.
//!
//! Escape bytes and other controls are stripped first. Without that, text
//! copied from a web page could carry `ESC [ 201 ~`, end the bracket early,
//! and have the rest run as typed commands. Line ends become carriage
//! returns, which is what the Enter key sends.

use alloc::vec::Vec;

use crate::term::Modes;

pub fn encode_paste(text: &str, modes: &Modes, out: &mut Vec<u8>) {
    if modes.bracketed_paste {
        out.extend_from_slice(b"\x1b[200~");
    }
    let mut prev_cr = false;
    for c in text.chars() {
        match c {
            '\n' if prev_cr => {}
            '\r' | '\n' => out.push(b'\r'),
            '\t' => out.push(b'\t'),
            c if c.is_control() => {}
            c => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
        prev_cr = c == '\r';
    }
    if modes.bracketed_paste {
        out.extend_from_slice(b"\x1b[201~");
    }
}
