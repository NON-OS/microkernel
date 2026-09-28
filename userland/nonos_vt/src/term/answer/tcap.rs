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

//! XTGETTCAP: terminfo capabilities by name, hex encoded both ways.

use alloc::string::String;
use alloc::vec::Vec;

use crate::term::state::Term;

fn hex_decode(s: &[u8]) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    let digit = |c: u8| (c as char).to_digit(16);
    s.chunks(2).map(|p| Some((digit(p[0])? * 16 + digit(p[1])?) as u8)).collect()
}

fn hex_encode(b: &[u8]) -> String {
    const H: &[u8; 16] = b"0123456789ABCDEF";
    b.iter().flat_map(|&x| [H[(x >> 4) as usize] as char, H[(x & 15) as usize] as char]).collect()
}

impl Term {
    pub(in crate::term) fn xtgettcap(&mut self, data: &[u8]) {
        for name in data.split(|&b| b == b';') {
            let Some(cap) = hex_decode(name) else { continue };
            let value: Option<&[u8]> = match cap.as_slice() {
                b"TN" | b"name" => Some(b"xterm-256color"),
                b"Co" | b"colors" => Some(b"256"),
                b"RGB" => Some(b"8/8/8"),
                _ => None,
            };
            let n = hex_encode(&cap);
            match value {
                Some(v) => {
                    let v = hex_encode(v);
                    self.reply_fmt(format_args!("\x1bP1+r{n}={v}\x1b\\"));
                }
                None => self.reply_fmt(format_args!("\x1bP0+r{n}\x1b\\")),
            }
        }
    }
}
