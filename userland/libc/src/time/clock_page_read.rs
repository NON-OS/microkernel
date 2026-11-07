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

//! Finds the clock page once per process and reads it.

use core::sync::atomic::{AtomicU64, Ordering};

use super::clock_page::{read, WORDS};
use crate::syscall::{call_raw, N_MK_CLOCK_PAGE};

const UNASKED: u64 = 0;
const ABSENT: u64 = 1;
static PAGE: AtomicU64 = AtomicU64::new(UNASKED);

fn page() -> Option<&'static [AtomicU64]> {
    let mut va = PAGE.load(Ordering::Relaxed);
    if va == UNASKED {
        let answer = call_raw(N_MK_CLOCK_PAGE, [0; 6]);
        // A page address is page-aligned; anything else, an errno included,
        // means no page.
        va = if answer > 0 && answer & 0xFFF == 0 { answer as u64 } else { ABSENT };
        PAGE.store(va, Ordering::Relaxed);
    }
    if va == ABSENT {
        return None;
    }
    // SAFETY: the kernel answered that this page is mapped read-only in this
    // address space for the life of the process (`MkClockPage`), and WORDS
    // u64s fit in it with u64 alignment.
    Some(unsafe { core::slice::from_raw_parts(va as *const AtomicU64, WORDS) })
}

#[cfg(target_arch = "x86_64")]
fn counter() -> u64 {
    // SAFETY: RDTSC reads the time-stamp counter; the page's user-counter
    // flag says the kernel leaves it readable at CPL 3.
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// The clock page's milliseconds since boot, or None to use the syscall.
#[cfg(target_arch = "x86_64")]
pub fn monotonic_ms() -> Option<u64> {
    let ms = read(page()?, counter);
    if ms.is_none() {
        PAGE.store(ABSENT, Ordering::Relaxed);
    }
    ms
}

#[cfg(not(target_arch = "x86_64"))]
pub fn monotonic_ms() -> Option<u64> {
    None
}
