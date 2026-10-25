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

use core::sync::atomic::AtomicBool;

use super::once::first;
use crate::arch::interrupt_controller::{broadcast_ipi, Ipi};
use crate::arch::power::{enter, PowerOff};
use crate::hardware::block_device::leave::{CRITICAL_TOTAL_MS, TOTAL_MS};
use crate::security::hardening::zerostate_shutdown_wipe;

/// Taken by the first terminate().
static ENDING: AtomicBool = AtomicBool::new(false);

/// Wipe, then leave. The only way out of a running system.
///
/// The order is the whole point and it is not negotiable:
///
/// 0. Once: a later caller (a second press, a critical battery, any caller
///    once a panic took its path) parks, so none stops the CPUs under a wipe.
/// 1. Leave: the disks flushed and their controllers told power is going,
///    each bounded and all within 15 s (leave.rs). The capsules that do it
///    must still run, so this comes before anything stops.
/// 2. Stop the other CPUs. A core still scheduling writes memory behind the
///    wipe, and whatever it writes survives into the next boot. Quiescing
///    first is what makes the wipe a snapshot rather than a race.
/// 3. Wipe. Process memory, the kernel heap, the key vault.
/// 4. Hand the machine to the firmware, which never returns.
///
/// The stop is best effort by design. On a uniprocessor boot there is nobody
/// to stop, and on a controller that refuses the broadcast the honest choice
/// is to wipe what this core can reach rather than to stay up holding
/// everything. A refusal costs coverage; refusing to shut down costs the lot.
pub(crate) fn terminate(off: PowerOff) -> ! {
    end(off, TOTAL_MS)
}

/// terminate() for a critical thermal zone or battery: leave within 6 s.
pub(crate) fn terminate_critical(off: PowerOff) -> ! {
    end(off, CRITICAL_TOTAL_MS)
}

fn end(off: PowerOff, leave_ms: u64) -> ! {
    if crate::boot::panic::panic_stage_taken() || !first(&ENDING) {
        park()
    }
    super::leave::run(off, leave_ms);
    let _ = broadcast_ipi(Ipi::Stop);
    zerostate_shutdown_wipe();
    enter(off)
}

/// A later caller's process given up, a tick at a time, interrupts on: a CPU
/// halted with them off would hold the first caller too, whose leave phase
/// rests between rounds. The first caller's stop takes this CPU as any.
fn park() -> ! {
    let mut round = 0u32;
    loop {
        crate::services::lifecycle::rest(round);
        round = round.saturating_add(1);
    }
}
