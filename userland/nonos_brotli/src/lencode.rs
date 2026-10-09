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
use crate::huff::Code;

/// The order code length code lengths come in (RFC 7932 3.5).
const ORDER: [usize; 18] = [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// The fixed code of those lengths, indexed by the next four bits.
const LEN_BITS: [u8; 16] = [2, 2, 2, 3, 2, 2, 2, 4, 2, 2, 2, 3, 2, 2, 2, 4];
const LEN_VALUE: [u8; 16] = [0, 4, 3, 2, 0, 4, 3, 1, 0, 4, 3, 2, 0, 4, 3, 5];

/// The prefix code of code lengths, its own lengths sent in ORDER
/// through the fixed code above, starting past `hskip` of them.
pub(crate) fn length_code(b: &mut Bits, hskip: usize) -> Result<Code, Error> {
    let (mut lens, mut space, mut used, mut last) = ([0u8; 18], 32i32, 0, 0);
    for &at in &ORDER[hskip..] {
        let (v, have) = b.peek(4);
        let n = LEN_BITS[v as usize] as u32;
        if n > have {
            return Err(Error::Truncated);
        }
        b.skip(n);
        lens[at] = LEN_VALUE[v as usize];
        if lens[at] != 0 {
            (space, used, last) = (space - (32 >> lens[at]), used + 1, at);
            if space <= 0 {
                break;
            }
        }
    }
    match (used, space) {
        (1, _) => Ok(Code::single(last as u16)),
        (_, 0) => Code::build(&lens),
        _ => Err(Error::Invalid),
    }
}
