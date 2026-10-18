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

//! The trampoline built once the kernel runs high, and each exposed CPU's
//! turn to use it.

use core::sync::atomic::Ordering;

use spin::Once;

use super::state::{self, EXPOSED};
use crate::arch::aarch64::mmu::{runs_high, trampoline};

pub(super) static REFUSED: Once<&'static str> = Once::new();

extern "C" {
    /// `asm/vectors.S`: where the trampoline's slots land.
    static __aarch64_tramp_targets: u8;
}

/// Called by every CPU that security::spectre finds exposed, once.
pub fn exposed_here() {
    EXPOSED.fetch_add(1, Ordering::AcqRel);
    state::switch_here();
}

/// Build the trampoline, on the boot CPU before any other CPU starts or any
/// user root exists, and turn it on here if this CPU is exposed. Its targets
/// are kernel text at the high alias, which user roots never map, so it is
/// built only where the kernel runs there.
pub fn activate(boot_exposed: bool) {
    if !runs_high() {
        REFUSED.call_once(|| "the kernel runs in TTBR0");
        return;
    }
    let targets = core::ptr::addr_of!(__aarch64_tramp_targets) as u64;
    if let Err(reason) = trampoline::install(targets) {
        REFUSED.call_once(|| reason);
        return;
    }
    state::built();
    if boot_exposed {
        state::switch_here();
    }
}
