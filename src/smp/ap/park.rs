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

//! An AP parking itself when the boot CPU has given up on its bring-up.

use crate::smp::state::CPU_DESCRIPTORS;

/// A point between two steps of `bring_up`, where this AP holds no lock.
///
/// If the boot CPU asked this AP to park (it missed its online deadline), it
/// answers `PARKED` and halts here with interrupts masked, for good. Parking
/// only here is what lets the boot CPU send INIT afterwards: a step that takes
/// the paging manager or the allocator holds its lock only inside the call,
/// and `bring_up` holds none across its own statements, which
/// `scripts/check_ap_checkpoints.py` holds it to.
#[inline]
pub(super) fn checkpoint(cpu_id: u32) {
    let claim = &CPU_DESCRIPTORS[cpu_id as usize].boot_claim;
    if !claim.park_asked() {
        return;
    }
    claim.ap_park();
    loop {
        // SAFETY: eK@nonos.systems - masks and halts this CPU only; it holds
        // no lock and owes no one an answer.
        unsafe { core::arch::asm!("cli", "hlt", options(nostack)) };
    }
}
