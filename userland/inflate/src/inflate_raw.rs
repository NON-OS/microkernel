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

//! Raw DEFLATE (RFC 1951) streams.

use alloc::vec::Vec;

use super::bits::Bits;
use super::dynamic::dynamic;
use super::fixed::fixed;
use super::out::Out;
use super::stored::stored;
use super::tables::{Codes, MAX_OUT};
use super::types::{End, Inflated};

/// The whole stream's output, or `None` when it is cut short, corrupt or
/// would exceed `MAX_OUT`. Bytes after the final block are ignored.
pub fn inflate(src: &[u8]) -> Option<Vec<u8>> {
    raw_partial(src, MAX_OUT).complete()
}

/// Decodes as much of `src` as it can, stopping at `cap` output bytes.
pub fn raw_partial(src: &[u8], cap: usize) -> Inflated {
    run(src, cap, src.len().saturating_mul(4))
}

/// The same, with room reserved up front for about `hint` output bytes.
pub(super) fn run(src: &[u8], cap: usize, hint: usize) -> Inflated {
    let mut b = Bits::new(src);
    let mut out = Out::new(cap, hint);
    let end = match blocks(&mut b, &mut out) {
        Ok(()) => End::Complete,
        Err(e) => e,
    };
    Inflated { out: out.finish(), end, used: b.consumed() }
}

fn blocks(b: &mut Bits, out: &mut Out) -> Result<(), End> {
    let mut c = Codes::new();
    loop {
        let last = b.bits(1)?;
        match b.bits(2)? {
            0 => stored(b, out)?,
            1 => fixed(b, out, &mut c)?,
            2 => dynamic(b, out, &mut c)?,
            _ => return Err(End::Corrupt),
        }
        if last == 1 {
            return Ok(());
        }
    }
}
