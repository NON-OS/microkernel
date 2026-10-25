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

//! Taking a queue back: emptied when halted, or skipped for a frame first
//! when the controller may be running it; and a retired run acknowledged.

use crate::env::{until, Clock, Mem, Regs};
use crate::facts::bounds::FRAME_MS;
use crate::facts::ed::{ED_K, HEAD_C, PTR_MASK};
use crate::facts::regs::{HC_FM_NUMBER, HC_INTERRUPT_STATUS, INT_WDH};

/// Empty a halted queue: HeadP to TailP, the halt cleared, the toggle
/// carry kept or reset. A halted ED is skipped, so its HeadP is ours.
pub fn clear_halt<M: Mem>(m: &M, ed: usize, keep_c: bool) {
    let (head, tail) = (m.rd(ed + 8), m.rd(ed + 4));
    let carry = if keep_c { head & HEAD_C } else { 0 };
    m.wr(ed + 8, tail & PTR_MASK | carry);
}

/// Take a queue back from a running controller: the ED skipped, a frame
/// let pass so the controller is done with it, emptied as `clear_halt`,
/// then its control word as it was.
pub fn settle<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, ed: usize, keep_c: bool) {
    let word = m.rd(ed);
    m.wr(ed, word | ED_K);
    let frame = r.r32(HC_FM_NUMBER) & 0xFFFF;
    until(c, FRAME_MS, || r.r32(HC_FM_NUMBER) & 0xFFFF != frame);
    clear_halt(m, ed, keep_c);
    m.wr(ed, word);
}

/// Retire an ED for another endpoint: skipped, a frame let pass, its queue
/// emptied onto `td` with the carry reset; it stays skipped.
pub fn park<M: Mem, R: Regs, C: Clock>(m: &M, r: &mut R, c: &C, ed: usize, td: usize) {
    m.wr(ed, m.rd(ed) | ED_K);
    let frame = r.r32(HC_FM_NUMBER) & 0xFFFF;
    until(c, FRAME_MS, || r.r32(HC_FM_NUMBER) & 0xFFFF != frame);
    m.wr(ed + 4, m.bus(td));
    m.wr(ed + 8, m.bus(td));
}

/// A run ended: its TDs went to the done queue, and the controller writes
/// the next done head only once this status is cleared.
pub fn retired<R: Regs>(r: &mut R) {
    r.w32(HC_INTERRUPT_STATUS, INT_WDH);
}
