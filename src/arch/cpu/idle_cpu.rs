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

use core::arch::asm;

// idle_cpu: enable interrupts atomically with the halt. When the platform
// cannot guarantee the LAPIC timer survives a halt (no ARAT and C1E not
// disabled), spin with interrupts enabled instead so the scheduler tick
// keeps arriving. The spin still lets any pending interrupt fire (sti) and
// `pause` keeps it cheap; the next scheduler pass re-checks the runqueue.
/// Wait for an interrupt. Returns with interrupts CLOSED on x86_64, whatever
/// they were going in. aarch64 and riscv64 return them open: a caller shared
/// between arches must set the state it needs itself, not assume either.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn idle_cpu() {
    use crate::arch::x86_64::interrupt::{cli, sti};
    if crate::arch::x86_64::interrupt::apic::idle_timer::halt_safe() {
        // SAFETY: STI's shadow keeps a pending wake inside the HLT. Not
        // `nomem`: the scheduler state read around it must not move across.
        unsafe {
            asm!("sti; hlt", options(nostack));
        }
    } else {
        sti();
        for _ in 0..4096 {
            core::hint::spin_loop();
        }
    }
    // Callers loop over scheduler state (run queue, process table) right
    // after idling and rely on the kernel-wide IF=0 discipline for those
    // locks; returning with interrupts still enabled lets the timer ISR
    // land inside the caller's next lock section and spin-deadlock the core.
    cli();
}

/// The wait `arch::idle` makes, masked going in. Returns with IRQs OPEN on
/// aarch64, whatever they were going in; x86_64 returns them closed. A caller
/// shared between arches must set the state it needs itself, not assume either.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn idle_cpu() {
    unsafe {
        asm!("msr daifset, #2", options(nomem, nostack));
    }
    crate::arch::aarch64::idle_states::wait();
    unsafe {
        asm!("msr daifclr, #2", options(nomem, nostack));
    }
}

/// Wait for an interrupt. Returns with interrupts OPEN on riscv64 (SIE set),
/// as on aarch64; x86_64 returns them closed. A caller shared between arches
/// must set the state it needs itself, not assume either.
#[cfg(target_arch = "riscv64")]
#[inline(always)]
pub fn idle_cpu() {
    unsafe {
        asm!("csrsi sstatus, 2", "wfi", options(nomem, nostack));
    }
}
