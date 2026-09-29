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

//! The decode loop for the bulk of a block: while at least 8 input bytes
//! remain and the output has `SLACK` room, every refill yields 56 bits and
//! no symbol needs a bounds or end-of-input check of its own. State lives
//! in locals so it stays in registers.

use super::bits::Bits;
use super::copy::matched;
use super::huff::{Table, EOB, KIND, LEN, LIT};
use super::out::{Out, SLACK};
use super::types::End;

/// Decodes until the block ends (`Some`), an error (`Some(Err)`), or the
/// input or output nears its end (`None`, for the careful loop to go on).
pub fn fast(
    b: &mut Bits,
    out: &mut Out,
    lit: &Table<1024>,
    dist: &Table<256>,
) -> Option<Result<(), End>> {
    let mut s = *b;
    let buf = &mut out.buf[..];
    let mut pos = out.pos;
    let (out_end, in_end) = (buf.len().saturating_sub(SLACK), s.d.len().saturating_sub(8));
    let res = loop {
        if pos > out_end || s.pos > in_end {
            break None;
        }
        s.refill();
        let e = lit.lookup(s.buf);
        if e & KIND == LIT && e & 0xFF != 0 {
            s.drop_bits(e & 0xFF);
            buf[pos] = (e >> 16) as u8;
            pos += 1;
            let e = lit.lookup(s.buf);
            if e & KIND == LIT && e & 0xFF != 0 {
                s.drop_bits(e & 0xFF);
                buf[pos] = (e >> 16) as u8;
                pos += 1;
            }
            continue;
        }
        let n = e & 0xFF;
        if n == 0 {
            break Some(Err(End::Corrupt));
        }
        s.drop_bits(n);
        match e & KIND {
            LEN => match matched(&mut s, dist, buf, pos, e) {
                Ok(n) => pos += n,
                Err(end) => break Some(Err(end)),
            },
            EOB => break Some(Ok(())),
            _ => break Some(Err(End::Corrupt)),
        }
    };
    *b = s;
    out.pos = pos;
    res
}
