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

use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use super::layout::{publish, Values, FLAG_USER_COUNTER, WORDS};
use super::mapping::{PERMS, VA};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout::DIRECTMAP_BASE;

static FRAME: AtomicU64 = AtomicU64::new(0);
static WRITER: Mutex<()> = Mutex::new(());

fn words(frame: u64) -> &'static [AtomicU64] {
    // SAFETY: `frame` is the 4 KiB frame `refresh` took from the allocator
    // for this page and never returns; the directmap maps all of RAM, and
    // WORDS u64s fit in the page with u64 alignment.
    unsafe { core::slice::from_raw_parts((DIRECTMAP_BASE + frame) as *const AtomicU64, WORDS) }
}

#[cfg(target_arch = "x86_64")]
fn user_counter() -> u64 {
    const CR4_TSD: u64 = 1 << 2;
    let cr4: u64;
    // SAFETY: reading CR4 has no side effect; the kernel runs at CPL 0.
    unsafe { core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags)) };
    if cr4 & CR4_TSD == 0 {
        FLAG_USER_COUNTER
    } else {
        0
    }
}

// No user-readable counter is promised on the other architectures yet;
// their libc reads the clock by syscall.
#[cfg(not(target_arch = "x86_64"))]
fn user_counter() -> u64 {
    0
}

/*
 * Writes the page from the clock's current state, taking its frame the
 * first time. Called after `sys::clock::init`; the values it copies are
 * set there once per boot. Without a frame there is no page, and every
 * capsule reads the clock by syscall.
 */
pub fn refresh() {
    // The riscv64 preview has no directmap until its VM layer is ported, so
    // the page cannot be written there. It stays absent and capsules read the
    // clock by syscall, as they do when no frame is free.
    if cfg!(target_arch = "riscv64") {
        return;
    }
    let _held = WRITER.lock();
    let mut frame = FRAME.load(Ordering::Acquire);
    if frame == 0 {
        let Some(f) = crate::memory::frame_alloc::allocate_frame() else {
            crate::log::log_warning!("[CLOCK] no clock page: no frame; capsules read the clock by syscall");
            return;
        };
        // SAFETY: a fresh 4 KiB frame, reached through the directmap, not yet
        // mapped anywhere else.
        unsafe { core::ptr::write_bytes((DIRECTMAP_BASE + f.as_u64()) as *mut u8, 0, 4096) };
        frame = f.as_u64();
        FRAME.store(frame, Ordering::Release);
    }
    let values = Values {
        flags: user_counter(),
        hz: super::super::TSC_HZ.load(Ordering::Relaxed),
        boot_count: super::super::BOOT_TSC.load(Ordering::Relaxed),
    };
    publish(words(frame), &values);
    crate::log::info!(
        "[CLOCK] page at {:#x} hz={} user-counter={}",
        VA,
        values.hz,
        if values.flags & FLAG_USER_COUNTER != 0 { "yes" } else { "no" }
    );
}

/// Whether `phys` is the clock page's frame, which no address space owns.
pub fn is_frame(phys: u64) -> bool {
    let f = FRAME.load(Ordering::Acquire);
    f != 0 && f == phys
}

/// Map the page into `asid`. False when there is no page or `VA` is taken
/// (an ELF segment placed there): the capsule then reads by syscall.
pub fn map_into(asid: u32) -> bool {
    let f = FRAME.load(Ordering::Acquire);
    if f == 0 || crate::memory::paging::manager::translate_in_asid(asid, VirtAddr::new(VA)).is_some() {
        return false;
    }
    crate::memory::paging::manager::map_page_in_asid(asid, VirtAddr::new(VA), PhysAddr::new(f), PERMS).is_ok()
}

/// Whether `asid` sees the clock page at `VA`.
pub fn mapped_in(asid: u32) -> bool {
    let f = FRAME.load(Ordering::Acquire);
    f != 0
        && crate::memory::paging::manager::translate_in_asid(asid, VirtAddr::new(VA))
            .map(|p| p.as_u64() & !0xFFF)
            == Some(f)
}
