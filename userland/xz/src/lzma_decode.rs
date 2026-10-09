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

//! One LZMA chunk: literals, matches and repeated matches until exactly
//! `unpacked` bytes have been produced (after Igor Pavlov's LzmaSpec.cpp).

use alloc::vec::Vec;

use super::lzma_copy::copy;
use super::lzma_dist::distance;
use super::lzma_literal::literal;
use super::lzma_model::Model;
use super::lzma_rep::{rep, Rep};
use super::range::Range;

const REP: usize = 0;

pub fn chunk(
    rc: &mut Range,
    m: &mut Model,
    out: &mut Vec<u8>,
    start: usize,
    unpacked: usize,
    dict: u32,
) -> Option<()> {
    let end = out.len().checked_add(unpacked)?;
    while out.len() < end {
        let pos = (out.len() - start) & ((1 << m.pb) - 1);
        let s = m.state;
        if rc.bit(&mut m.is_match[(s << 4) + pos]) == 0 {
            literal(rc, m, out, start)?;
            continue;
        }
        let len = if rc.bit(&mut m.is_rep[REP][s]) == 1 {
            if out.len() == start {
                return None;
            }
            match rep(rc, m, s, pos) {
                Rep::Short => {
                    copy(out, start, m.rep[0], 1, dict)?;
                    continue;
                }
                Rep::Len(len) => len,
            }
        } else {
            m.rep = [0, m.rep[0], m.rep[1], m.rep[2]];
            let len = m.len.decode(rc, pos);
            m.state = if s < 7 { 7 } else { 10 };
            m.rep[0] = distance(rc, m, len)?;
            len
        };
        let len = len as usize + 2;
        if out.len() + len > end {
            return None;
        }
        copy(out, start, m.rep[0], len, dict)?;
    }
    Some(())
}
