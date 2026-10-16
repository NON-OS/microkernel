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

//! The trampoline's own tables and data page, in the image: one page each,
//! page aligned, so no other kernel data shares a page a user root maps.
//! Written once, by the boot CPU, before any root points at them.

use core::sync::atomic::{AtomicU64, Ordering};

use super::geometry::{chain, Chain};
use super::leaves::bits;
use crate::arch::aarch64::mmu::image_pa;
use crate::arch::aarch64::security::kernel_guarded;

#[repr(C, align(4096))]
struct Page([AtomicU64; 512]);

impl Page {
    const fn new() -> Self {
        Self([const { AtomicU64::new(0) }; 512])
    }

    fn pa(&'static self) -> u64 {
        image_pa(self as *const Self as u64)
    }
}

static L1: Page = Page::new();
static L2: Page = Page::new();
static L3: Page = Page::new();
static DATA: Page = Page::new();

extern "C" {
    /// `asm/epd1_trampoline.S`: the code page.
    static __epd1_trampoline: u8;
}

/// Fill the tables and the data page, which names `targets`, the address
/// the kernel runs `__aarch64_tramp_targets` at, and answer the chain. The
/// root's word is the caller's to place.
pub(super) fn fill(targets: u64) -> Chain {
    let code = image_pa(core::ptr::addr_of!(__epd1_trampoline) as u64);
    let words = chain([L1.pa(), L2.pa(), L3.pa()], code, DATA.pa(), bits(kernel_guarded()));
    DATA.0[0].store(targets, Ordering::Relaxed);
    L3.0[0].store(words.code, Ordering::Relaxed);
    L3.0[1].store(words.data, Ordering::Relaxed);
    L2.0[0].store(words.l2, Ordering::Relaxed);
    L1.0[0].store(words.l1, Ordering::Relaxed);
    words
}
