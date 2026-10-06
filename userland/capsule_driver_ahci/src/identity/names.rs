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

//! The disk's model number and serial number, as IDENTIFY DEVICE gives them
//! (ACS-3, 7.12.7): ATA strings, two characters per word with the first in
//! the word's high byte, padded with spaces. The bytes are the drive's own,
//! so anything that is not printable ASCII is shown as '?', never passed on.

use crate::constants::identify::{IDENTIFY_WORDS, MODEL_BYTES, SERIAL_BYTES, W_MODEL, W_SERIAL};

/// What the installer shows for a disk besides its size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Names {
    pub model: [u8; MODEL_BYTES],
    pub model_len: u8,
    pub serial: [u8; SERIAL_BYTES],
    pub serial_len: u8,
}

impl Names {
    pub const fn empty() -> Self {
        Self { model: [0; MODEL_BYTES], model_len: 0, serial: [0; SERIAL_BYTES], serial_len: 0 }
    }

    pub fn model(&self) -> &[u8] {
        &self.model[..self.model_len as usize]
    }

    pub fn serial(&self) -> &[u8] {
        &self.serial[..self.serial_len as usize]
    }
}

pub fn names(words: &[u16; IDENTIFY_WORDS]) -> Names {
    let mut n = Names::empty();
    n.model_len = ata_string(&words[W_MODEL..W_MODEL + MODEL_BYTES / 2], &mut n.model) as u8;
    n.serial_len = ata_string(&words[W_SERIAL..W_SERIAL + SERIAL_BYTES / 2], &mut n.serial) as u8;
    n
}

/// Unpack `words` into `out` (two bytes per word, high byte first), trim the
/// space and NUL padding on both ends, and give the length left. The trimmed
/// text starts at `out[0]`; the rest of `out` is zero.
fn ata_string(words: &[u16], out: &mut [u8]) -> usize {
    let mut raw = [0u8; MODEL_BYTES];
    let n = core::cmp::min(words.len() * 2, core::cmp::min(out.len(), raw.len()));
    for (i, b) in raw[..n].iter_mut().enumerate() {
        let w = words[i / 2];
        *b = if i % 2 == 0 { (w >> 8) as u8 } else { w as u8 };
    }
    let pad = |b: &u8| *b == b' ' || *b == 0;
    let start = raw[..n].iter().position(|b| !pad(b)).unwrap_or(n);
    let end = raw[..n].iter().rposition(|b| !pad(b)).map_or(start, |i| i + 1);
    out.fill(0);
    for (o, &b) in out.iter_mut().zip(&raw[start..end]) {
        *o = if (0x20..0x7f).contains(&b) { b } else { b'?' };
    }
    end - start
}
