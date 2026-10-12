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

//! A qTD's words (facts::qtd), written and read. Pure over the region.

use crate::env::Mem;
use crate::facts::qh::LINK_T;
use crate::facts::qtd::*;

/// One qTD: its PID, its toggle (control's; others take the QH's), and
/// its buffer.
#[derive(Clone, Copy)]
pub struct Qtd {
    pub pid: u32,
    pub toggle: bool,
    pub buf: usize,
    pub len: usize,
}

/// Write `q` at `at`, active, linked to `next` and, after a short IN
/// packet, to `alt` (None: the end).
pub fn write<M: Mem>(m: &M, at: usize, q: Qtd, next: Option<usize>, alt: Option<usize>) {
    let link = |to: Option<usize>| to.map_or(LINK_T, |t| m.bus(t));
    m.wr(at, link(next));
    m.wr(at + 4, link(alt));
    for page in 0..5 {
        let addr = match (q.len, page) {
            (0, _) => 0,
            (_, 0) => m.bus(q.buf),
            (_, p) => (m.bus(q.buf) & !0xFFF) + 0x1000 * p,
        };
        m.wr(at + 12 + 4 * page as usize, addr);
    }
    let dt = if q.toggle { TOK_DT } else { 0 };
    let bytes = (q.len as u32 & TOK_BYTES_MASK) << TOK_BYTES_SHIFT;
    // The token last: the qTD is whole before it is active.
    m.wr(at + 8, dt | bytes | TOK_CERR_3 | q.pid << TOK_PID_SHIFT | TOK_ACTIVE);
}

pub fn token<M: Mem>(m: &M, at: usize) -> u32 {
    m.rd(at + 8)
}

/// The bytes a retired qTD of `len` moved: those its total did not leave.
pub fn moved<M: Mem>(m: &M, at: usize, len: usize) -> usize {
    let left = ((token(m, at) >> TOK_BYTES_SHIFT) & TOK_BYTES_MASK) as usize;
    len.saturating_sub(left)
}
