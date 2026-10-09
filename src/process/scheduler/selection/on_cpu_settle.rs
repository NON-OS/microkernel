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

//! A CPU settling into the stack it runs on: off the one it left, and owning
//! the one it is on.

use core::sync::atomic::Ordering;

use super::on_cpu::{this_cpu, LEAVING, OWNED, TRACKED};

/// This CPU is off the stack it was leaving.
pub(crate) fn release_leaving() {
    if !TRACKED {
        return;
    }
    let slot = &LEAVING[this_cpu()];
    let left = slot.load(Ordering::Relaxed);
    if left != 0 {
        slot.store(0, Ordering::SeqCst);
        /*
         * A waker that found the pid still named here woke nobody for it.
         */
        super::on_cpu_wake::left_claimable(left);
    }
}

/// Adopt `pid` as this CPU's own when it runs one no switch recorded: the
/// first process, which the boot path starts by hand.
pub(crate) fn adopt_current(pid: u32) {
    if !TRACKED || pid == 0 {
        return;
    }
    let slot = &OWNED[this_cpu()];
    if slot.load(Ordering::Relaxed) == 0 {
        slot.store(pid, Ordering::SeqCst);
    }
}
