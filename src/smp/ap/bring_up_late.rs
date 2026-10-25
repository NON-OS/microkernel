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
//! The second half of an AP's bring-up, after its per-CPU base: extended
//! state, stack guards, the IDT, the syscall registers and its timer.

use super::park::checkpoint;

/// Every step is followed by a `checkpoint`, as in `bring_up`, and no
/// statement here holds a lock past its own call.
///
/// # Safety
/// Once per AP, from `bring_up`, interrupts masked.
pub(super) unsafe fn finish(cpu_id: u32) -> super::user_setup::Prepared {
    // The trampoline leaves CR4.OSXSAVE clear and XCR0 unset, so AVX code
    // faulted here and an area saved on the boot CPU could not be restored.
    // SAFETY: eK@nonos.systems - once, during this AP's bring-up, interrupts off.
    unsafe { crate::arch::x86_64::cpu::xstate::mirror_on_ap() };
    checkpoint(cpu_id);

    // Its own block: the slot was handed to this CPU and is never reused.
    let _ = crate::arch::x86_64::gdt::arm_ap_guards(cpu_id);
    checkpoint(cpu_id);

    // The IDT the BSP runs on, built by `interrupts::init_idt`. This pointed
    // at the second IDT under `arch::x86_64::idt`, whose `init` has no caller,
    // so an AP loaded a table nothing filled in and triple-faulted on its
    // first timer tick. `load` only writes IDTR against the BSP.s table.
    crate::interrupts::idt::load_idt();
    checkpoint(cpu_id);

    /*
     * The syscall registers and CR4/CR0/EFER are per CPU. Set here, with
     * interrupts still masked; the result is reported once they are open.
     */
    let user = super::user_setup::prepare();
    checkpoint(cpu_id);

    // The BSP registered the IRQ-0 handler; each AP arms its own LAPIC timer.
    crate::arch::x86_64::interrupt::apic::preemption::install_on_ap();
    checkpoint(cpu_id);

    user
}
