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

//! A device's commitment as its transcript line records it:
//! `device <key hex> <w0> <w1> <w2> <w3>`. Read only after `Registry::recompute`
//! took the same text, so every line is well formed and every word canonical.
//! The last line for a key is the one the registry kept, since enrolling again
//! under a key replaces its commitment. Keys are compared as bytes, decoded
//! the way the registry decodes them.

/// The commitment's four words as 32 little-endian bytes.
pub fn commitment_of(text: &str, id: &[u8; 32]) -> Option<[u8; 32]> {
    text.lines().rev().find_map(|l| device_line(l, id))
}

fn device_line(line: &str, id: &[u8; 32]) -> Option<[u8; 32]> {
    let mut f = line.split_whitespace();
    if f.next()? != "device" || !same_key(f.next()?, id) {
        return None;
    }
    let mut out = [0u8; 32];
    for word in out.chunks_exact_mut(8) {
        word.copy_from_slice(&f.next()?.parse::<u64>().ok()?.to_le_bytes());
    }
    f.next().is_none().then_some(out)
}

fn same_key(hex: &str, id: &[u8; 32]) -> bool {
    hex.len() == 64
        && id.iter().enumerate().all(|(i, b)| {
            let pair = hex.get(2 * i..2 * i + 2);
            pair.and_then(|p| u8::from_str_radix(p, 16).ok()) == Some(*b)
        })
}
