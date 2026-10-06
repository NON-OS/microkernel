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

use super::bits::Bits;
use super::codes::{codes, symbol};
use super::huff::Table;
use super::huff_build::build;
use super::meta;
use super::out::Out;
use super::tables::{Codes, ORDER};
use super::types::End;

/// A block that sends its own codes (RFC 1951 3.2.7).
pub fn dynamic(b: &mut Bits, out: &mut Out, c: &mut Codes) -> Result<(), End> {
    let hlit = b.bits(5)? as usize + 257;
    let hdist = b.bits(5)? as usize + 1;
    let hclen = b.bits(4)? as usize + 4;
    if hlit > 286 || hdist > 30 {
        return Err(End::Corrupt);
    }
    let mut cl = [0u8; 19];
    for &i in ORDER.iter().take(hclen) {
        cl[i] = b.bits(3)? as u8;
    }
    build(&mut c.cl, &cl, meta::plain)?;
    let lens = lengths(b, &c.cl, hlit + hdist)?;
    build(&mut c.lit, &lens[..hlit], meta::litlen)?;
    build(&mut c.dist, &lens[hlit..hlit + hdist], meta::dist)?;
    codes(b, out, &c.lit, &c.dist)
}

/// The literal/length and distance code lengths, run-length coded with
/// the code-length code: 16 repeats the previous length, 17 and 18 zeros.
fn lengths(b: &mut Bits, cl: &Table<128>, total: usize) -> Result<[u8; 316], End> {
    let mut lens = [0u8; 316];
    let mut n = 0;
    while n < total {
        b.refill();
        let sym = (symbol(b, cl)? >> 16) as u8;
        let (val, rep) = match sym {
            0..=15 => (sym, 1),
            16 => {
                (*n.checked_sub(1).and_then(|p| lens.get(p)).ok_or(End::Corrupt)?, 3 + b.bits(2)?)
            }
            17 => (0, 3 + b.bits(3)?),
            18 => (0, 11 + b.bits(7)?),
            _ => return Err(End::Corrupt),
        };
        let end = n + rep as usize;
        if end > total {
            return Err(End::Corrupt);
        }
        lens[n..end].fill(val);
        n = end;
    }
    Ok(lens)
}
