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
use crate::huff::{Code, SLOW};

impl Code {
    /// Match the leading bits of `v` (first bit lowest) against codes of
    /// up to `max` bits, as the canonical walk of RFC 1951 3.2.2.
    pub(crate) fn walk(&self, v: u32, max: u32) -> Option<(u16, u32)> {
        let (mut code, mut first, mut index) = (0u32, 0u32, 0u32);
        for len in 1..=max {
            code |= (v >> (len - 1)) & 1;
            let n = self.count[len as usize] as u32;
            if code < first + n {
                return Some((self.syms[(index + code - first) as usize], len));
            }
            index += n;
            first = (first + n) << 1;
            code <<= 1;
        }
        None
    }

    pub(crate) fn read(&self, b: &mut Bits) -> Result<u16, Error> {
        let (v, have) = b.peek(15);
        let (sym, len) = match self.fast[(v & 0xff) as usize] {
            SLOW => self.walk(v, 15).ok_or(Error::Invalid)?,
            e => (e >> 4, (e & 15) as u32),
        };
        if len > have {
            return Err(Error::Truncated);
        }
        b.skip(len);
        Ok(sym)
    }
}
