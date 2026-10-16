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

//! A general TD's words (facts::td), written and read. Pure over the
//! region.

use crate::env::Mem;
use crate::facts::td::*;

/// One TD: its direction, its toggle (None: the ED's carry), whether a
/// short packet is allowed, and its buffer.
#[derive(Clone, Copy)]
pub struct Td {
    pub dp: u32,
    pub toggle: Option<bool>,
    pub round: bool,
    pub buf: usize,
    pub len: usize,
}

/// Write `td` at `at`, not yet retired, linked to the TD at `next`.
pub fn write<M: Mem>(m: &M, at: usize, td: Td, next: usize) {
    let toggle = match td.toggle {
        None => 0,
        Some(false) => TD_T_DATA0,
        Some(true) => TD_T_DATA1,
    };
    let round = if td.round { TD_R } else { 0 };
    let cc = CC_NOT_ACCESSED << TD_CC_SHIFT;
    m.wr(at, round | td.dp << TD_DP_SHIFT | TD_DI_NONE | toggle | cc);
    let (cbp, be) = match td.len {
        0 => (0, 0),
        n => (m.bus(td.buf), m.bus(td.buf + n - 1)),
    };
    m.wr(at + 4, cbp);
    m.wr(at + 8, m.bus(next));
    m.wr(at + 12, be);
}

/// The condition code the controller left (CC_NOT_ACCESSED: not retired).
pub fn cc<M: Mem>(m: &M, at: usize) -> u32 {
    m.rd(at) >> TD_CC_SHIFT
}

/// The bytes a retired TD of `len` from `buf` moved: all once its CBP is
/// 0, else up to the CBP a short packet left. A CBP outside the buffer is
/// no count of the bytes moved, so none are taken.
pub fn moved<M: Mem>(m: &M, at: usize, buf: usize, len: usize) -> usize {
    match (len, m.rd(at + 4)) {
        (0, _) => 0,
        (_, 0) => len,
        (_, cbp) => match cbp.checked_sub(m.bus(buf)) {
            Some(n) if (n as usize) < len => n as usize,
            _ => 0,
        },
    }
}
