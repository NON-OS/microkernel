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

//! The way into every exception this kernel does not resume from.
//!
//! Both terminal paths, the vectors with no handler and the synchronous faults
//! the trap contract classifies as fatal, come through [`enter`] before they
//! print. It masks D, A, I and F on this CPU, gives EL1 the FP/SIMD registers,
//! and counts how deep this CPU is in the terminal path. The first time, the
//! caller reports. If the report itself faults, the second entry says so in a
//! line that needs no formatting and the CPU parks. Any deeper entry parks
//! without writing, because by then writing is what faults.
//!
//! FP/SIMD is granted because the report is compiled code that uses the vector
//! registers, and lazy FP leaves CPACR_EL1.FPEN trapping EL1 as well as EL0
//! while a task that has not touched them since it was switched in is current.
//! The CPU is parking, so whose register contents this exposes does not matter.
//!
//! The count is kept per CPU, by roster index, so that two CPUs failing at once
//! both report. It is read and written with plain loads and stores, never an
//! atomic read-modify-write: the path runs with the MMU off early in boot, when
//! this memory is Device memory and exclusive access to it is not guaranteed.

use core::arch::asm;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::aarch64::cpu::id::cpu_id;
use crate::sys::serial::core::write_fatal_line;

/// CPACR_EL1.FPEN = 0b11: FP/SIMD usable at EL1 and EL0 without trapping.
const CPACR_FPEN_FULL: u64 = 0b11 << 20;

static DEPTH: [AtomicU8; MAX_CPUS] = [const { AtomicU8::new(0) }; MAX_CPUS];

const NESTED: &[u8] = b"[TRAP] fault while reporting a fault, this CPU is parked";

/// Mask interrupts, grant EL1 the vector registers, and say whether this CPU
/// may report. False means the caller must go straight to its halt.
pub(crate) fn enter() -> bool {
    // SAFETY: masking D, A, I and F at EL1 is always permitted and only stops
    // further asynchronous exceptions on this CPU. Widening CPACR_EL1.FPEN
    // cannot fault, and the ISB makes it visible before any vector register
    // use that follows. No `nomem`: the compiler must not move a load that
    // may use a vector register above the grant.
    unsafe {
        asm!(
            "msr daifset, #0xf",
            "mrs {t}, cpacr_el1",
            "orr {t}, {t}, {fpen}",
            "msr cpacr_el1, {t}",
            "isb",
            t = out(reg) _,
            fpen = in(reg) CPACR_FPEN_FULL,
            options(nostack, preserves_flags)
        );
    }
    let depth = &DEPTH[slot()];
    let seen = depth.load(Ordering::Relaxed);
    depth.store(seen.saturating_add(1), Ordering::Relaxed);
    match seen {
        0 => true,
        1 => {
            write_fatal_line(NESTED);
            false
        }
        _ => false,
    }
}

/// This CPU's roster index. Every CPU the kernel starts has its own; before the
/// roster is latched it is 0, which is right because only the boot CPU runs
/// then. The lookup is a load-acquire and plain loads, nothing exclusive.
fn slot() -> usize {
    cpu_id() % MAX_CPUS
}
