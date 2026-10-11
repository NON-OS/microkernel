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

//! The unit's own interrupts: the event queue and global errors, wired as
//! edge SPIs. With them, a fault is in the log when it happens rather than
//! at the next driver call; without them (a tree that wires none, or a GIC
//! that refuses the line), draining on every call still reports it.

use crate::arch::aarch64::gic;
use crate::sys::serial::Line;

use super::mmio::Regs;
use super::regs::{IRQ_CTRL, IRQ_CTRLACK, IRQ_CTRL_EVENTQ, IRQ_CTRL_GERROR};
use super::state::SMMU;
use super::wait;

/// Route both SPIs to `on_interrupt`, then let the unit raise them.
pub(super) fn wire(regs: Regs, eventq: u32, gerror: u32) {
    let mut enable = 0;
    for (intid, bit) in [(eventq, IRQ_CTRL_EVENTQ), (gerror, IRQ_CTRL_GERROR)] {
        if intid == 0 {
            continue;
        }
        let routed = gic::configure_spi(intid, true).is_ok()
            && gic::register_irq_handler(intid, on_interrupt).is_ok();
        if routed {
            gic::enable_irq(intid);
            enable |= bit;
        } else {
            Line::new().str(b"[SMMU] interrupt ").dec(u64::from(intid)).str(b" not routed").end();
        }
    }
    regs.write32(IRQ_CTRL, enable);
    if !wait::until(|| regs.read32(IRQ_CTRLACK) == enable) {
        crate::sys::serial::println(b"[SMMU] IRQ_CTRL not acknowledged; faults reported per call");
    }
}

/// Never blocks: if a driver call holds the unit, that call drains the
/// queue and reports global errors on its way out.
fn on_interrupt(_intid: u32) {
    if let Some(mut guard) = SMMU.try_lock() {
        if let Some(smmu) = guard.as_mut() {
            smmu.eventq.drain(smmu.regs);
            super::gerror::report(smmu.regs);
        }
    }
}
