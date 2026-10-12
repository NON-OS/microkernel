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

//! An interrupt IN endpoint: one qTD always armed on its own periodic QH,
//! run in micro-frame 0 of every frame while the device NAKs (split below
//! a TT, completed in micro-frames 2 to 4); the toggle is
//! the QH's. A poll looks once: still active is "nothing yet"; done hands
//! back the report and arms the next.

use crate::env::{Clock, Mem, Regs};
use crate::facts::qtd::PID_IN;
use crate::sched::layout::{intr_buf, intr_qh, intr_qtd, INTR_BUF_LEN};
use crate::sched::link::Link;
use crate::sched::qh::{chars, empty};
use crate::sched::qtd::{write, Qtd};
use crate::sched::queue::start;
use crate::sched::take::park;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntrEp {
    pub addr: u8,
    pub ep: u8,
    pub mps: u16,
    /// Which of the INTR_MAX QHs, qTDs and buffers is this endpoint's.
    pub index: usize,
    pub len: usize,
    pub link: Link,
}

/// A parked QH set for `ep`, given a micro-frame, and armed.
pub fn open<M: Mem>(m: &M, ep: &IntrEp) {
    let at = intr_qh(ep.index);
    m.wr(at + 4, ep.link.chars(chars(ep.addr, ep.ep, ep.mps, false), false));
    empty(m, at, false);
    m.wr(at + 8, ep.link.caps(true));
    arm(m, ep);
}

pub fn arm<M: Mem>(m: &M, ep: &IntrEp) {
    let (i, len) = (ep.index, ep.len.min(INTR_BUF_LEN));
    write(m, intr_qtd(i), Qtd { pid: PID_IN, toggle: false, buf: intr_buf(i), len }, None, None);
    start(m, intr_qh(i), intr_qtd(i));
}

/// Stop the endpoint's QH, for another endpoint to open.
pub fn stop<M: Mem, R: Regs, C: Clock>((m, r, c): (&M, &mut R, &C), op: u32, index: usize) {
    park((m, r, c), op, intr_qh(index));
}
