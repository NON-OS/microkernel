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

//! The two per-architecture steps of a cache flush: one line out, then the
//! barrier that orders the flushes before what follows.

pub(super) fn flush_line(line: u64) {
    // SAFETY: the caller hands a range the kernel maps; cleaning and
    // invalidating a line changes no memory contents.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("clflush [{}]", in(reg) line, options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dc civac, {}", in(reg) line, options(nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = line;
}

pub(super) fn fence() {
    // SAFETY: a barrier has no memory operands.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("dsb sy", options(nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
