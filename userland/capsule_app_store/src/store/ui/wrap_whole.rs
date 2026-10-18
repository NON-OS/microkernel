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
//! A line wrapped at its spaces, where a word wider than the pane, an
//! address in full, is carried on letter by letter across the edge rather
//! than painted past it: every byte of it is on some line.

use alloc::vec::Vec;

use super::text::width_of;

pub fn wrap_whole(text: &[u8], room: u32, px: f32) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    let mut line: Vec<u8> = Vec::new();
    for word in text.split(|b| *b == b' ').filter(|w| !w.is_empty()) {
        let gap: &[u8] = if line.is_empty() { b"" } else { b" " };
        if width_of(&[&line[..], gap, word].concat(), px) <= room {
            line.extend_from_slice(gap);
            line.extend_from_slice(word);
            continue;
        }
        if width_of(word, px) <= room {
            out.push(core::mem::replace(&mut line, word.to_vec()));
            continue;
        }
        line.extend_from_slice(gap);
        for c in core::str::from_utf8(word).unwrap_or("").chars() {
            let before = line.len();
            line.extend_from_slice(c.encode_utf8(&mut [0u8; 4]).as_bytes());
            if width_of(&line, px) > room && before > 0 {
                let carried = line.split_off(before);
                out.push(core::mem::replace(&mut line, carried));
            }
        }
    }
    if !line.is_empty() {
        out.push(line);
    }
    out
}
