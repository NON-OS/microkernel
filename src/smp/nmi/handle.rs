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

//! The NMI handler's first step.

use core::sync::atomic::Ordering;

use super::send::{HALT_ALL, KICKED};
use crate::smp::types::CpuState;

/// Halt for a fatal stop, serve a shootdown round, and say whether the NMI is
/// accounted for; `false` leaves it to the handler's checks for a hardware
/// cause. NMI-safe, see the module notes.
///
/// A kick that coalesced with another NMI, or whose flag an earlier NMI on the
/// same cpu already consumed, reaches the hardware checks as an unknown NMI.
/// That costs a warning line, never a missed round: the round is served here
/// either way, through its own pending flag.
pub fn on_nmi() -> bool {
    if HALT_ALL.load(Ordering::Acquire) {
        halt_here();
    }
    let served = crate::memory::paging::manager::handle_shootdown_ipi();
    let kicked = match crate::smp::cpu_id::try_cpu_id() {
        Some(cpu) => KICKED.get(cpu).is_some_and(|f| f.swap(false, Ordering::AcqRel)),
        None => false,
    };
    served || kicked
}

fn halt_here() -> ! {
    if let Some(cpu) = crate::smp::cpu_id::try_cpu_id().and_then(crate::smp::cpu::get_cpu) {
        cpu.set_state(CpuState::Halted);
    }
    crate::arch::halt_loop()
}
