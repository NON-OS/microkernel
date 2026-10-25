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

//! Taking a QH back. An async QH is unlinked from the ring, the async
//! advance doorbell rung so the controller holds no copy of it, then its
//! overlay is emptied (or its characteristics rewritten) and it is linked
//! back. A periodic QH loses its micro-frame mask and a frame passes. A
//! halted QH is skipped by the controller, so it is emptied at once.

use super::layout::{qh, ring_prev};
use super::qh::{empty, qh_link};
use crate::env::{until, Clock, Mem, Regs};
use crate::facts::bounds::DOORBELL_MS;
use crate::facts::regs::{CMD_IAAD, FRINDEX, STS_IAA, USBCMD, USBSTS};

/// Ring the async advance doorbell and see it answered.
pub fn doorbell<R: Regs, C: Clock>(r: &mut R, c: &C, op: u32) {
    r.w32(op + USBSTS, STS_IAA);
    let cmd = r.r32(op + USBCMD);
    r.w32(op + USBCMD, cmd | CMD_IAAD);
    until(c, DOORBELL_MS, || r.r32(op + USBSTS) & STS_IAA != 0);
    r.w32(op + USBSTS, STS_IAA);
}

/// Ring QH `i` taken back: emptied (the toggle kept or reset) and, when
/// characteristics and capabilities are given, set for another endpoint.
pub fn take_back<M: Mem, R: Regs, C: Clock>(
    (m, r, c): (&M, &mut R, &C),
    op: u32,
    i: usize,
    keep_dt: bool,
    set: Option<(u32, u32)>,
) {
    let (this, prev) = (qh(i), ring_prev(i));
    m.wr(prev, m.rd(this));
    doorbell(r, c, op);
    if let Some((ch, caps)) = set {
        m.wr(this + 4, ch);
        m.wr(this + 8, caps);
    }
    empty(m, this, keep_dt);
    m.wr(prev, qh_link(m, this));
}

/// A periodic QH stopped: no micro-frame mask, a frame let pass (FRINDEX
/// counts micro-frames), then emptied with DATA0 next.
pub fn park<M: Mem, R: Regs, C: Clock>((m, r, c): (&M, &mut R, &C), op: u32, at: usize) {
    m.wr(at + 8, m.rd(at + 8) & !0xFF);
    let start = r.r32(op + FRINDEX);
    until(c, 3, || r.r32(op + FRINDEX).wrapping_sub(start) & 0x3FFF >= 8);
    empty(m, at, false);
}

/// A halted QH emptied, the toggle kept or reset.
pub fn clear_halt<M: Mem>(m: &M, at: usize, keep_dt: bool) {
    empty(m, at, keep_dt);
}
