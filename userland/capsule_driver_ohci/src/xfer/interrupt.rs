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

//! An interrupt IN endpoint: one TD always armed on its own ED, which the
//! controller tries every frame while the device NAKs; the toggle is the
//! ED's carry. A poll looks once: still queued is "nothing yet"; done
//! hands back the report and arms the next on the other place
//! (interrupt_poll.rs).

use crate::env::{Clock, Mem, Regs};
use crate::facts::td::DP_IN;
use crate::sched::ed::{control as ed_word, Target};
use crate::sched::layout::{intr_buf, intr_ed, intr_td, INTR_BUF_LEN};
use crate::sched::queue::start;
use crate::sched::take::park;
use crate::sched::td::{write, Td};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntrEp {
    pub t: Target,
    /// Which of the INTR_MAX EDs, TD pairs and buffers is this endpoint's.
    pub index: usize,
    pub mps: u16,
    pub len: usize,
    /// The place the queue rests on.
    pub at: u8,
}

pub fn arm<M: Mem, R: Regs>(m: &M, r: &mut R, ep: &IntrEp) {
    let (i, len) = (ep.index, ep.len.min(INTR_BUF_LEN));
    let td = Td { dp: DP_IN, toggle: None, round: true, buf: intr_buf(i), len };
    write(m, intr_td(i, ep.at), td, intr_td(i, 1 - ep.at));
    m.wr(intr_ed(i), ed_word(ep.t, ep.mps, false));
    start(m, r, intr_ed(i), intr_td(i, 1 - ep.at), 0);
}

/// Take the endpoint's ED back, skipped and empty on its first place, for
/// another endpoint to use from there.
pub fn stop<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, index: usize) {
    park(m, r, c, intr_ed(index), intr_td(index, 0));
}
