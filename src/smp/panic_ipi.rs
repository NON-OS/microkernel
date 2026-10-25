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

use super::types::CpuState;
use crate::arch::interrupt_controller::{broadcast_ipi, Ipi};

/// Stop every other cpu for a fatal halt.
///
/// On the multi-core image this is an NMI: the panic vector waits for its
/// target to unmask interrupts, and a cpu spinning masked on a lock the
/// halting cpu holds never does, so "halt the machine" left it running. The
/// vector remains the fallback when no NMI could be sent, and is what the
/// single-cpu image sends, where there is nobody to reach either way.
pub fn send_panic_ipi() {
    if cfg!(feature = "nonos-smp") && super::nmi::halt_others() {
        return;
    }
    let _ = broadcast_ipi(Ipi::Panic);
}

/// How long the panic path waits for the other CPUs to halt: an NMI is taken
/// within microseconds, so this is far past any CPU that is going to.
const HALT_WAIT_SPINS: u32 = 10_000_000;

/// Whether every other CPU has said it halted, within HALT_WAIT_SPINS.
/// Atomic loads of each CPU's state; a CPU that never came up is not waited
/// for. `false` costs nothing but the answer: the wipe goes ahead, because
/// staying up holding everything is the worse outcome.
pub fn others_halted() -> bool {
    let me = super::cpu_id::try_cpu_id();
    let mut spins = 0u32;
    loop {
        let mut running = false;
        let mut id = 0;
        while let Some(cpu) = super::cpu::get_cpu(id) {
            let live = matches!(
                cpu.state(),
                CpuState::Starting | CpuState::Online | CpuState::GoingOffline
            );
            if live && Some(id) != me {
                running = true;
            }
            id += 1;
        }
        if !running {
            return true;
        }
        if spins >= HALT_WAIT_SPINS {
            return false;
        }
        spins += 1;
        core::hint::spin_loop();
    }
}
