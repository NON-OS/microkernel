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

//! A frame header (RFC 8878 3.1.1.1), after the magic number.

use super::limits::MAX_OUT;

pub struct Header {
    pub used: usize,
    pub content: Option<usize>,
    pub checksum: bool,
}

pub fn header(d: &[u8]) -> Option<Header> {
    let fhd = *d.first()?;
    let (fcs, single) = (fhd >> 6, fhd & 0x20 != 0);
    // Bit 3 is reserved and must be zero.
    if fhd & 0x08 != 0 {
        return None;
    }
    let mut at = 1 + usize::from(!single);
    let le = |from: usize, n: usize| {
        let b = d.get(from..from + n)?;
        Some(b.iter().rev().fold(0u64, |v, &x| v << 8 | u64::from(x)))
    };
    let did = [0, 1, 2, 4][usize::from(fhd & 3)];
    // A dictionary is not something this decoder has; refuse, never guess.
    if le(at, did)? != 0 {
        return None;
    }
    at += did;
    let size = match fcs {
        0 => usize::from(single),
        1 => 2,
        2 => 4,
        _ => 8,
    };
    let content = match size {
        0 => None,
        2 => Some(le(at, 2)? + 256),
        n => Some(le(at, n)?),
    };
    if content.is_some_and(|c| c > MAX_OUT as u64) {
        return None;
    }
    Some(Header {
        used: at + size,
        content: content.map(|c| c as usize),
        checksum: fhd & 0x04 != 0,
    })
}
