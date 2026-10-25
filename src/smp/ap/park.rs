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

#[cfg(all(
    any(feature = "nonos-smp-park-proof", feature = "nonos-smp-stuck-proof"),
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!(
    "nonos-smp-park-proof and nonos-smp-stuck-proof hold CPU 1 back from bring-up on \
     purpose; they are boot proofs and never part of a production build."
);

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

/// `nonos-smp-park-proof` and `nonos-smp-stuck-proof` only: CPU 1 is made slow
/// in bring-up, long enough to miss its online deadline, so a boot shows what
/// the boot CPU does with it. With the park proof it checkpoints every 10 ms
/// while slow, as any step of `bring_up` would be followed by one, and is
/// parked and reset. With the stuck proof it stalls three seconds with no
/// checkpoint, as a step wedged inside itself would, so the boot CPU's park
/// request goes unanswered and the CPU stays a shootdown target.
#[cfg(any(feature = "nonos-smp-park-proof", feature = "nonos-smp-stuck-proof"))]
pub(super) fn proof_stall(cpu_id: u32) {
    if cpu_id != 1 {
        return;
    }
    let per_ms = crate::sys::timer::tsc::tsc_frequency() / 1000;
    if per_ms == 0 {
        return;
    }
    let total_ms: u64 = if cfg!(feature = "nonos-smp-stuck-proof") { 3000 } else { 1500 };
    let start = crate::arch::read_time_counter();
    let mut next_check = start;
    while crate::arch::read_time_counter().wrapping_sub(start) < total_ms * per_ms {
        if cfg!(feature = "nonos-smp-park-proof")
            && crate::arch::read_time_counter().wrapping_sub(next_check) >= 10 * per_ms
        {
            next_check = crate::arch::read_time_counter();
            checkpoint(cpu_id);
        }
        core::hint::spin_loop();
    }
}
