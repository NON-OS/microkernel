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
use crate::prefix::MAX_ALPHABET;

/// A code sent as code lengths, themselves prefix coded, with 16 to
/// repeat the last nonzero length and 17 to repeat zero.
pub(crate) fn read(b: &mut Bits, alphabet: usize, hskip: usize) -> Result<Code, Error> {
    let lens_code = crate::lencode::length_code(b, hskip)?;
    let mut lens = [0u8; MAX_ALPHABET];
    let (mut sym, mut space, mut prev, mut rep, mut rep_len) = (0, 32768i32, 8u8, 0usize, 0u8);
    while sym < alphabet && space > 0 {
        let c = lens_code.read(b)? as u8;
        if c < 16 {
            rep = 0;
            if c != 0 {
                (lens[sym], prev) = (c, c);
                space -= 32768 >> c;
            }
            sym += 1;
            continue;
        }
        let (extra, len) = if c == 16 { (2, prev) } else { (3, 0) };
        if rep_len != len {
            (rep, rep_len) = (0, len);
        }
        let old = rep;
        let grown = if rep > 0 { (rep - 2) << extra } else { 0 };
        rep = grown + b.read(extra)? as usize + 3;
        let delta = rep - old;
        if sym + delta > alphabet {
            return Err(Error::Invalid);
        }
        if len != 0 {
            lens[sym..sym + delta].fill(len);
            space -= (delta as i32) << (15 - len);
        }
        sym += delta;
    }
    match space {
        0 => Code::build(&lens[..alphabet]),
        _ => Err(Error::Invalid),
    }
}
