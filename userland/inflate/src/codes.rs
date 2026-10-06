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
use super::fast::fast;
use super::huff::{Table, EOB, KIND, LEN, LIT};
use super::out::Out;
use super::types::End;

/// Decodes one block's literal/length and distance codes up to its end of
/// block: the fast loop while it can, then one symbol at a time with every
/// check. A refill before each symbol leaves 56 bits while the input lasts,
/// enough for a length, its distance and both extra fields.
pub fn codes(b: &mut Bits, out: &mut Out, lit: &Table<1024>, dist: &Table<256>) -> Result<(), End> {
    loop {
        if let Some(done) = fast(b, out, lit, dist) {
            return done;
        }
        b.refill();
        let e = symbol(b, lit)?;
        match e & KIND {
            LIT => out.lit((e >> 16) as u8)?,
            LEN => {
                let len = (e >> 16) as usize + extra(b, e)?;
                let d = symbol(b, dist)?;
                if d & KIND != LIT {
                    return Err(End::Corrupt);
                }
                out.copy((d >> 16) as usize + extra(b, d)?, len)?;
            }
            EOB => return Ok(()),
            _ => return Err(End::Corrupt),
        }
    }
}

/// Decodes and consumes one code. A missing code with the input used up
/// may be the start of one the input never delivered.
#[inline(always)]
pub fn symbol<const N: usize>(b: &mut Bits, t: &Table<N>) -> Result<u32, End> {
    let e = t.lookup(b.buf);
    let n = e & 0xFF;
    if n == 0 || n > b.cnt {
        let short = n > b.cnt || (b.drained() && b.cnt < 15);
        return Err(if short { End::Truncated } else { End::Corrupt });
    }
    b.drop_bits(n);
    Ok(e)
}

/// The value of an entry's extra bits.
#[inline(always)]
fn extra(b: &mut Bits, e: u32) -> Result<usize, End> {
    let n = (e >> 8) & 0xF;
    if n > b.cnt {
        return Err(End::Truncated);
    }
    let v = (b.buf & ((1u64 << n) - 1)) as usize;
    b.drop_bits(n);
    Ok(v)
}
