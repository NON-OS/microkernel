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

//! The trampoline put into the kernel's root, which TTBR0 and TTBR1 share,
//! and the word every user root made after that carries in the same slot.

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

use super::geometry::SLOT;
use super::tables;
use crate::arch::aarch64::mmu::state;

static ROOT_WORD: AtomicU64 = AtomicU64::new(0);

/// Map the trampoline into the kernel's root, its data page naming
/// `targets`. Refused if the slot is not empty, which would mean something
/// else maps there and a user root would show it.
pub(in crate::arch::aarch64) fn install(targets: u64) -> Result<(), &'static str> {
    // SAFETY: the boot CPU, before any user root exists; this is the only
    // live borrow of the kernel's root.
    let l0 = unsafe { state::l0() };
    if l0.entry(SLOT) != 0 {
        return Err("the kernel's top-level slot 508 is in use");
    }
    let words = tables::fill(targets);
    // SAFETY: barriers only. The tables are complete in memory before the
    // root names them, and the root's word before any CPU is told of it.
    unsafe { asm!("dsb ishst", options(nostack, preserves_flags)) };
    l0.set_entry(SLOT, words.root);
    unsafe { asm!("dsb ish", "isb", options(nostack, preserves_flags)) };
    ROOT_WORD.store(words.root, Ordering::Release);
    Ok(())
}

/// The slot and the word a user root carries there, once installed.
pub(in crate::arch::aarch64) fn root_word() -> Option<(usize, u64)> {
    let word = ROOT_WORD.load(Ordering::Acquire);
    (word != 0).then_some((SLOT, word))
}
