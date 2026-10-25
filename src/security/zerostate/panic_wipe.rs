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

//! What a panic wipes before the machine goes off.
//!
//! A panic stops the machine with every byte it held still in DRAM, and
//! `terminate`'s wipe cannot run here: it takes the process table, the pager
//! and the allocator, and the panic may have been raised holding any of them.
//! So the order is:
//!
//! 1. Stop the other CPUs (the panic handler's NMI or IPI) and wait, for a
//!    bounded time, for each to say it has halted.
//! 2. Read what soft-off needs (`arch::power::prepare`), and drop the RAM log
//!    if nobody holds it: the last lock this path tries, and it never waits.
//! 3. Wipe: the kernel heap, by the extent the allocator keeps in two
//!    atomics, minus the panic path's own stack and the kernel image
//!    (`panic_plan`). No lock, no allocation and no log from here on.
//! 4. Soft-off from what step 2 read (`arch::power::enter_prepared`).
//!
//! The heap is where the key vault's map, IPC payloads, the file caches and
//! the session volume's key live, so they go with it. Process memory is not
//! reached: its frames come from the page allocator, and finding them takes
//! the locks above (docs/security/privacy-guarantees.md says what that
//! leaves). Page tables come from the page allocator too, never the heap, so
//! the wipe does not reach the tables the CPU is walking.

use super::panic_plan::{plan, Span, MAX_SPANS};
use super::panic_stack::stack_keep;
use crate::memory::layout::KSTACK_SIZE;
use crate::security::hardening::memory_sanitization::erase::secure_zero;

/// Wipe and power off. Called once, from the panic handler, after everything
/// it prints has been printed and the other CPUs have been stopped
/// (`smp::others_halted` says whether they confirmed it).
pub(crate) fn panic_wipe_and_off() -> ! {
    let off = crate::arch::power::prepare();
    let _ = crate::log::wipe_ram_log();

    if let Some((start, size)) = crate::memory::heap::get_allocator().extent() {
        let heap = Span { start: start as u64, end: (start as u64).saturating_add(size as u64) };
        let sp = crate::arch::stack_pointer();
        let stack = stack_keep(sp, KSTACK_SIZE as u64);
        let mut keep = [Span { start: 0, end: 0 }; 5];
        keep[0] = stack;
        for (slot, s) in keep[1..].iter_mut().zip(crate::memory::layout::kernel_sections().iter()) {
            *slot = Span { start: s.start, end: s.end };
        }
        let mut spans = [Span { start: 0, end: 0 }; MAX_SPANS];
        if let Some(n) = plan(heap, &keep, &mut spans) {
            for s in &spans[..n] {
                // SAFETY: each span lies inside the heap's window, which the
                // allocator mapped read-write for the kernel and set once at
                // init; the other CPUs were stopped, and no span overlaps the
                // stack this runs on or the kernel image (`panic_plan`).
                secure_zero(s.start as *mut u8, (s.end - s.start) as usize);
            }
        }
    }
    crate::arch::power::enter_prepared(off)
}
