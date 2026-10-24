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

//! A queue head's words (facts::qh), built. Pure over the region.

use crate::env::Mem;
use crate::facts::qh::*;
use crate::facts::qtd::TOK_DT;

/// A link to the QH at `off`.
pub fn qh_link<M: Mem>(m: &M, off: usize) -> u32 {
    m.bus(off) | LINK_QH
}

/// A high-speed endpoint's characteristics: its address, endpoint and
/// packet size; the toggle from each qTD (control) or kept in the QH.
pub const fn chars(addr: u8, ep: u8, mps: u16, dtc: bool) -> u32 {
    let dtc = if dtc { CH_DTC } else { 0 };
    (addr as u32 & 0x7F)
        | (ep as u32 & 0xF) << CH_EP_SHIFT
        | CH_EPS_HIGH
        | dtc
        | (mps as u32 & 0x7FF) << CH_MPS_SHIFT
        | CH_RL
}

/// The QH at `off` with no qTDs, linked to `link`.
pub fn init<M: Mem>(m: &M, off: usize, link: u32, chars: u32, caps: u32) {
    m.wr(off, link);
    m.wr(off + 4, chars);
    m.wr(off + 8, caps);
    empty(m, off, false);
}

/// The overlay emptied: no current qTD, none next, not active or halted;
/// the toggle kept, or DATA0 next.
pub fn empty<M: Mem>(m: &M, off: usize, keep_dt: bool) {
    let dt = m.rd(off + QH_TOKEN) & TOK_DT;
    m.wr(off + QH_CURRENT, 0);
    m.wr(off + QH_NEXT, LINK_T);
    m.wr(off + QH_ALT, LINK_T);
    m.wr(off + QH_TOKEN, if keep_dt { dt } else { 0 });
    for w in 0..5 {
        m.wr(off + QH_TOKEN + 4 + 4 * w, 0);
    }
}
