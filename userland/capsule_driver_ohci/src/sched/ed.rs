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

//! An endpoint descriptor's words (facts::ed), built. Pure over the region.

use crate::env::Mem;
use crate::facts::ed::{ED_EN_SHIFT, ED_K, ED_MPS_MASK, ED_MPS_SHIFT, ED_S};

/// Who an ED is for: the device's address and endpoint, and its speed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Target {
    pub addr: u8,
    pub ep: u8,
    pub low_speed: bool,
}

/// An ED's control word: direction taken from each TD, general TDs.
pub const fn control(t: Target, mps: u16, skip: bool) -> u32 {
    let mut w = (t.addr as u32 & 0x7F)
        | (t.ep as u32 & 0xF) << ED_EN_SHIFT
        | (mps as u32 & ED_MPS_MASK) << ED_MPS_SHIFT;
    if t.low_speed {
        w |= ED_S;
    }
    if skip {
        w |= ED_K;
    }
    w
}

/// An ED skipped, its queue empty and resting on `td`, linked to the ED
/// at bus address `next` (0: the end of its list).
pub fn init<M: Mem>(m: &M, ed: usize, td: usize, next: u32) {
    m.wr(ed, ED_K);
    m.wr(ed + 4, m.bus(td));
    m.wr(ed + 8, m.bus(td));
    m.wr(ed + 12, next);
}
