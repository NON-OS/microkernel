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

//! Each CPU's key slot, which the exception vectors reach through
//! TPIDR_EL1: words 0-1 the IA key of the process this CPU next enters EL0
//! with, words 2-3 the kernel's. TPIDR_EL1 stays 0 until the slot is bound,
//! and the vectors take 0 to mean no key switching on this CPU. EL0 cannot
//! read TPIDR_EL1.

use core::arch::asm;
use core::mem::size_of;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::arch::aarch64::boot::stack::MAX_CPUS;

#[repr(C, align(32))]
struct Slot {
    user: [AtomicU64; 2],
    kernel: [AtomicU64; 2],
}

static SLOTS: [Slot; MAX_CPUS] = [const {
    Slot { user: [const { AtomicU64::new(0) }; 2], kernel: [const { AtomicU64::new(0) }; 2] }
}; MAX_CPUS];

/// Give CPU `cpu` its slot, filled with the kernel key, and point its
/// TPIDR_EL1 at it. False for an index past the table.
pub(super) fn bind(cpu: usize, kernel: [u64; 2]) -> bool {
    let Some(slot) = SLOTS.get(cpu) else { return false };
    for (word, value) in slot.kernel.iter().zip(kernel) {
        word.store(value, Ordering::Relaxed);
    }
    // SAFETY: TPIDR_EL1 is this CPU's to use at EL1 and nothing else in the
    // kernel reads it; it now names a static that lives as long as the CPU.
    unsafe {
        asm!("dmb ish", "msr tpidr_el1, {}", in(reg) slot as *const Slot, options(nostack));
    }
    true
}

/// The key this CPU installs on its next entry to EL0 through
/// `aarch64_enter_user` or `aarch64_resume_user`.
pub(super) fn set_user(key: [u64; 2]) {
    let at: usize;
    // SAFETY: reading TPIDR_EL1 at EL1 has no effect.
    unsafe { asm!("mrs {}, tpidr_el1", out(reg) at, options(nomem, nostack, preserves_flags)) };
    if at == 0 {
        return;
    }
    // Only `bind` writes TPIDR_EL1, with the address of a slot in `SLOTS`;
    // found again by its index, so no number is taken for a pointer.
    let off = at.checked_sub(SLOTS.as_ptr() as usize).filter(|off| off % size_of::<Slot>() == 0);
    let index = off.map(|off| off / size_of::<Slot>());
    let Some(slot) = index.and_then(|i| SLOTS.get(i)) else { return };
    for (word, value) in slot.user.iter().zip(key) {
        word.store(value, Ordering::Relaxed);
    }
}
