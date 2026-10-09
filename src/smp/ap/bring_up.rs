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
//! its timer.

use crate::smp::state::CPU_DESCRIPTORS;

/// # Safety
/// Once per AP from `ap_entry`, its claim passed, interrupts masked.
pub(super) unsafe fn bring_up(cpu_id: u32) -> super::user_setup::Prepared {
    // This AP's LAPIC comes out of INIT/SIPI software-disabled. The mode and
    // MMIO mapping were adopted from the BSP before the SIPI, so only the
    // per-CPU register programming runs here.
    // SAFETY: eK@nonos.systems - runs once on this AP before any interrupt is
    // enabled, and touches only this CPU's own LAPIC registers.
    CPU_DESCRIPTORS[cpu_id as usize].set_stage(crate::smp::Stage::ApEntered);
    unsafe { crate::arch::x86_64::interrupt::apic::init_ap_lapic() };
    let apic_id = crate::arch::interrupt_controller::local_id();

    // Before anything else that could ask which CPU it is running on. The
    // answer comes out of this block through the per-CPU register, and that
    // register is installed here; until it is, this AP would be answered with
    // the boot CPU's number and would read and write the boot CPU's state.
    // `cpu_id` is the index the BSP wrote into this AP's boot context, so it
    // is known without having to look it up.
    crate::smp::percpu::init_ap(cpu_id as usize);

    // SAFETY: eK@nonos.systems - GDT and TSS must be loaded before anything
    // that can fault. `cpu_id` indexes this AP's own per-CPU structures, which
    // the BSP allocated before releasing it.
    unsafe {
        let _ = crate::arch::x86_64::cpu::init_ap(cpu_id as u16, apic_id);
    }
    crate::arch::set_percpu_base(crate::smp::percpu::current().self_ptr);

    // The trampoline leaves CR4.OSXSAVE clear and XCR0 unset, so AVX code
    // faulted here and an area saved on the boot CPU could not be restored.
    // SAFETY: eK@nonos.systems - once, during this AP's bring-up, interrupts off.
    unsafe { crate::arch::x86_64::cpu::xstate::mirror_on_ap() };

    // Its own block: the slot was handed to this CPU and is never reused.
    let _ = crate::arch::x86_64::gdt::arm_ap_guards(cpu_id);

    // The IDT the BSP runs on, built by `interrupts::init_idt`. This pointed
    // at the second IDT under `arch::x86_64::idt`, whose `init` has no caller,
    // so an AP loaded a table nothing filled in and triple-faulted on its
    // first timer tick. `load` only writes IDTR against the BSP.s table.
    crate::interrupts::idt::load_idt();

    /*
     * The syscall registers and CR4/CR0/EFER are per CPU. Set here, with
     * interrupts still masked; the result is reported once they are open.
     */
    let user = super::user_setup::prepare();

    // The BSP registered the IRQ-0 handler; each AP arms its own LAPIC timer.
    crate::arch::x86_64::interrupt::apic::preemption::install_on_ap();

    CPU_DESCRIPTORS[cpu_id as usize].set_stage(crate::smp::Stage::LapicArmed);
    user
}
