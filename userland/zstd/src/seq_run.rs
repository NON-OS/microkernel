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

//! Decoding sequences and executing them: literals copied, then a match
//! copied from earlier in the frame (RFC 8878 3.1.1.3.2.2 and 3.1.1.4).

use alloc::vec::Vec;

use super::back::Back;
use super::context::Context;
use super::limits::BLOCK_MAX;
use super::seq_codes::{literal_length, match_length};
use super::seq_rep::resolve;

pub fn run(
    d: &[u8],
    n: usize,
    ctx: &mut Context,
    lits: &[u8],
    out: &mut Vec<u8>,
    start: usize,
) -> Option<()> {
    let (ll, of, ml) = (ctx.ll.as_ref()?, ctx.of.as_ref()?, ctx.ml.as_ref()?);
    let mut bits = Back::new(d)?;
    let mut s = [ll.log, of.log, ml.log].map(|l| bits.read(l.into()) as usize);
    let (mut rep, mut lit, block) = (ctx.rep, 0usize, out.len());
    for i in 0..n {
        let (cl, co, cm) = (*ll.cells.get(s[0])?, *of.cells.get(s[1])?, *ml.cells.get(s[2])?);
        let value = (1usize << co.sym) + bits.read(co.sym.into()) as usize;
        let (base, extra) = match_length(cm.sym)?;
        let mlen = base as usize + bits.read(extra.into()) as usize;
        let (base, extra) = literal_length(cl.sym)?;
        let llen = base as usize + bits.read(extra.into()) as usize;
        let offset = resolve(&mut rep, value, llen)?;
        if i + 1 < n {
            s[0] = usize::from(cl.base) + bits.read(cl.bits.into()) as usize;
            s[2] = usize::from(cm.base) + bits.read(cm.bits.into()) as usize;
            s[1] = usize::from(co.base) + bits.read(co.bits.into()) as usize;
        }
        if out.len() - block + llen + mlen > BLOCK_MAX {
            return None;
        }
        out.extend_from_slice(lits.get(lit..lit + llen)?);
        lit += llen;
        if offset > out.len() - start {
            return None;
        }
        for _ in 0..mlen {
            out.push(out[out.len() - offset]);
        }
    }
    if !bits.done() {
        return None;
    }
    let tail = lits.get(lit..)?;
    if out.len() - block + tail.len() > BLOCK_MAX {
        return None;
    }
    out.extend_from_slice(tail);
    ctx.rep = rep;
    Some(())
}
