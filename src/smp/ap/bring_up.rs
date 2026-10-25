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

//! An AP's own CPU state, set up with interrupts masked, from its LAPIC to
//! its per-CPU base; `bring_up_late` goes on from there to its timer.

use super::park::checkpoint;
use crate::smp::state::CPU_DESCRIPTORS;

/// Every step is followed by a `checkpoint`, where an AP the boot CPU has
/// given up on parks. No statement here holds a lock past its own call.
///
/// # Safety
/// Once per AP from `ap_entry`, its claim passed, interrupts masked.
pub(super) unsafe fn bring_up(cpu_id: u32) -> super::user_setup::Prepared {
    // This AP's LAPIC comes out of INIT/SIPI software-disabled. The mode and
    // MMIO mapping were adopted from the BSP before the SIPI, so only the
    // per-CPU register programming runs here.
    // SAFETY: eK@nonos.systems - runs once on this AP before any interrupt is
    // enabled, and touches only this CPU's own LAPIC registers.
    CPU_DESCRIPTORS[cpu_id as usize].set_stage(crate::smp::Stage::ApEntered);
    checkpoint(cpu_id);
    #[cfg(any(feature = "nonos-smp-park-proof", feature = "nonos-smp-stuck-proof"))]
    super::park_proof::proof_stall(cpu_id);
    checkpoint(cpu_id);
    unsafe { crate::arch::x86_64::interrupt::apic::init_ap_lapic() };
    checkpoint(cpu_id);
    let apic_id = crate::arch::interrupt_controller::local_id();

    // Before anything else that could ask which CPU it is running on. The
    // answer comes out of this block through the per-CPU register, and that
    // register is installed here; until it is, this AP would be answered with
    // the boot CPU's number and would read and write the boot CPU's state.
    // `cpu_id` is the index the BSP wrote into this AP's boot context, so it
    // is known without having to look it up.
    crate::smp::percpu::init_ap(cpu_id as usize);
    checkpoint(cpu_id);

    // SAFETY: eK@nonos.systems - GDT and TSS must be loaded before anything
    // that can fault. `cpu_id` indexes this AP's own per-CPU structures, which
    // the BSP allocated before releasing it.
    unsafe {
        let _ = crate::arch::x86_64::cpu::init_ap(cpu_id as u16, apic_id);
    }
    checkpoint(cpu_id);
    crate::arch::set_percpu_base(crate::smp::percpu::current().self_ptr);
    checkpoint(cpu_id);

    // SAFETY: eK@nonos.systems - same AP, same claim, interrupts still masked.
    let user = unsafe { super::bring_up_late::finish(cpu_id) };
    checkpoint(cpu_id);

    CPU_DESCRIPTORS[cpu_id as usize].set_stage(crate::smp::Stage::LapicArmed);
    user
}
