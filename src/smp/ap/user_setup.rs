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

//! What an application processor needs before it may run user code.
//!
//! The `syscall` registers and the ring-0 restrictions are per CPU, and the
//! boot CPU setting its own does nothing for the others. An AP that has not
//! got both stays online for interrupts and shootdowns but never takes a
//! process off the run queue.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::memory::mmu::{MmuResult, ProtectionFlags};
use crate::smp::MAX_CPUS;

static READY: [AtomicBool; MAX_CPUS] = [const { AtomicBool::new(false) }; MAX_CPUS];

pub(super) struct Prepared {
    syscall: bool,
    protection: MmuResult<ProtectionFlags>,
}

/// Program this CPU's registers. Before `sti`, and silent: printing takes
/// the serial lock, which must not be contended with interrupts masked.
pub(super) fn prepare() -> Prepared {
    Prepared {
        syscall: crate::arch::x86_64::syscall::manager::init_ap().is_ok(),
        protection: crate::memory::mmu::apply_protection_this_cpu(),
    }
}

/// Report what `prepare` got, and allow user code only when all of it held.
pub(super) fn finish(cpu_id: u32, prepared: Prepared) {
    crate::memory::mmu::report_protection_cpu(cpu_id, &prepared.protection);
    let protected =
        prepared.protection.as_ref().map_or(false, crate::memory::mmu::protection_matches_boot);
    if prepared.syscall && protected {
        READY[cpu_id as usize % MAX_CPUS].store(true, Ordering::Release);
        return;
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] cpu=").dec(cpu_id as u64);
    l.str(b" runs no user code syscall=").dec(prepared.syscall as u64);
    l.str(b" protection=").dec(protected as u64);
    l.end();
}

/// Whether `cpu_id` passed `finish`.
pub(super) fn is_ready(cpu_id: u32) -> bool {
    READY[cpu_id as usize % MAX_CPUS].load(Ordering::Acquire)
}
