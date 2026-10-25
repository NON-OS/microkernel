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

//! Writing a chain of TDs: each linked to the next depth first, so the
//! whole chain runs within a frame when the device keeps up; the last ends
//! the queue and asks for an interrupt. Pure over the region.

use crate::env::Mem;
use crate::facts::td::{CTRL_SPD, LINK_DEPTH, LINK_TERMINATE, TD_LEN};
use crate::sched::td::{ctrl_word, state, token, Target, TdState};

/// One TD of a chain: its PID, data toggle, buffer offset and length.
#[derive(Clone, Copy)]
pub struct Step {
    pub pid: u8,
    pub toggle: bool,
    pub buf: usize,
    pub len: usize,
    /// Stop the queue on a short packet (IN data).
    pub spd: bool,
}

/// Write `steps` as TDs from `base`; returns how many.
pub fn write<M: Mem>(m: &M, base: usize, t: Target, steps: &[Step]) -> usize {
    for (i, s) in steps.iter().enumerate() {
        let at = base + i * TD_LEN;
        let last = i + 1 == steps.len();
        let link = if last { LINK_TERMINATE } else { m.bus(at + TD_LEN) | LINK_DEPTH };
        let ctrl = ctrl_word(t, last) | if s.spd { CTRL_SPD } else { 0 };
        let buf = if s.len == 0 { 0 } else { m.bus(s.buf) };
        m.wr(at + 4, ctrl);
        m.wr(at + 8, token(s.pid, t, s.toggle, s.len));
        m.wr(at + 12, buf);
        // The link last: a TD is only reachable once it is whole.
        m.wr(at, link);
    }
    steps.len()
}

/// The bytes a TD asked for, from its token's MaxLen.
pub fn asked<M: Mem>(m: &M, td: usize) -> usize {
    (((m.rd(td + 8) >> 21) + 1) & 0x7FF) as usize
}

/// The bytes the TDs `from..=upto` moved, each held to what it asked
/// for: ActLen is the controller's word, and one past MaxLen would carry
/// the count beyond the buffer the TDs were given.
pub fn moved<M: Mem>(m: &M, base: usize, from: usize, upto: usize) -> usize {
    (from..=upto)
        .map(|i| match state(m.rd(base + i * TD_LEN + 4)) {
            TdState::Done(n) => n.min(asked(m, base + i * TD_LEN)),
            _ => 0,
        })
        .sum()
}
