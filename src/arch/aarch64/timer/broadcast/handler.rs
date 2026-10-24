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

//! The frame's interrupt: wake every CPU that is due, then hold the next.

use super::state::{BOOK, FRAME};
use crate::arch::aarch64::boot::multicore::roster;
use crate::arch::aarch64::cpu::pack_affinity;
use crate::arch::aarch64::interrupt_controller::send_ipi;
use crate::arch::interrupt_controller::Ipi;

pub(super) fn on_fire(_intid: u32) {
    let Some(frame) = FRAME.get() else {
        return;
    };
    let mut book = BOOK.lock();
    let next = book.expire(frame.count(), wake);
    // Earliest of those still asleep. If it is already past by now, the
    // level interrupt is asserted again at once and this runs again.
    frame.hold(next);
}

/// A reschedule SGI: pending at the GIC, it ends the CPU's idle state, and
/// its own timer, put back with its context, fires for the deadline.
fn wake(cpu: usize) {
    if let Some(affinity) = roster::affinity_of(cpu) {
        let _ = send_ipi(pack_affinity(affinity), Ipi::Reschedule);
    }
}
