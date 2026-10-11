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

use core::sync::atomic::Ordering;

use crate::arch::aarch64::gic;
use crate::process::scheduler::dispatch::try_wake::try_wake_process;

use super::pending_find;

// Called from the GIC dispatch path with the live intid. Bumps the
// per-grant pending counter and masks the line so a level-triggered
// SPI cannot re-fire before the capsule's `MkIrqAck`. Edge sources
// are unaffected by the mask but still see the counter advance.
pub(super) fn handle(intid: u32) {
    let Some(e) = pending_find::find_by_intid(intid) else { return };
    let prev = e.pending.fetch_add(1, Ordering::AcqRel);
    if prev == u64::MAX {
        e.overflow.fetch_add(1, Ordering::Relaxed);
    }
    gic::disable_irq(intid);
    // Try only: the code this interrupt cut into may hold the waiter's
    // scheduler state. A wake that cannot land now goes back for the
    // next delivery; the waiter's own timeout bounds the delay.
    let waiter = e.waiter.swap(0, Ordering::AcqRel);
    if waiter != 0 && !try_wake_process(waiter) {
        let _ = e.waiter.compare_exchange(0, waiter, Ordering::AcqRel, Ordering::Relaxed);
    }
}
