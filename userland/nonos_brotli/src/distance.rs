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

use crate::bits::Bits;
use crate::error::Error;

/// Change to the last or second last distance for codes 4..=15.
const DELTA: [i64; 6] = [-1, 1, -2, 2, -3, 3];

/// The four last distances, newest last (RFC 7932 4).
pub(crate) struct Ring {
    d: [u32; 4],
}

impl Ring {
    pub(crate) fn new() -> Ring {
        Ring { d: [16, 15, 11, 4] }
    }

    pub(crate) fn last(&self) -> u32 {
        self.d[3]
    }

    pub(crate) fn push(&mut self, v: u32) {
        self.d.copy_within(1..4, 0);
        self.d[3] = v;
    }

    /// The distance that `code` stands for, reading its extra bits.
    pub(crate) fn distance(
        &self,
        b: &mut Bits,
        code: usize,
        (postfix, direct): (u32, usize),
    ) -> Result<u32, Error> {
        if code < 16 {
            let base = match code {
                0..=3 => return Ok(self.d[3 - code]),
                4..=9 => self.d[3],
                _ => self.d[2],
            };
            let v = base as i64 + DELTA[(code - 4) % 6];
            return if v > 0 { Ok(v as u32) } else { Err(Error::Invalid) };
        }
        if code < 16 + direct {
            return Ok((code - 15) as u32);
        }
        let x = code - direct - 16;
        let nbits = 1 + (x >> (postfix + 1)) as u32;
        let offset = ((2 + (x >> postfix & 1)) << nbits) - 4;
        let extra = b.read(nbits)? as usize;
        let low = x & ((1 << postfix) - 1);
        Ok((((offset + extra) << postfix) + low + direct + 1) as u32)
    }
}
