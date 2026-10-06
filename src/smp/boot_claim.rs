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

//! Who decides whether a slow AP is still coming.
//!
//! The trampoline holds one boot context, and the boot CPU rewrites it for
//! each AP in turn. An AP that is woken but slow to run would read whatever
//! the context says when it finally gets there, and if the boot CPU has moved
//! on, that is the next AP's stack and cpu number: two CPUs on one stack.
//!
//! So the boot CPU and the AP race for one word. The AP claims it as the
//! first thing it does in Rust, having already read its context; the boot CPU
//! claims it when its wait expires. Exactly one wins. An AP that loses parks
//! itself without touching anything shared, and the boot CPU that wins sends
//! it INIT, which holds it in wait-for-SIPI, before the context is reused.
//! An AP that won has finished with the context, so it can be reused at once
//! even if that AP is slow to reach Online.
//!
//! Host-checked by path from `kernel_proofs`; it uses nothing but core.

use core::sync::atomic::{AtomicU32, Ordering};

const WAITING: u32 = 0;
const ENTERED: u32 = 1;
const ABANDONED: u32 = 2;

pub struct BootClaim(AtomicU32);

impl BootClaim {
    pub const fn new() -> Self {
        Self(AtomicU32::new(WAITING))
    }

    /// Reset before the AP is sent its STARTUP. Only the boot CPU calls this,
    /// and only for a slot it has never handed out.
    pub fn arm(&self) {
        self.0.store(WAITING, Ordering::Release);
    }

    /// The AP's claim. False means the boot CPU gave up on it first and the
    /// AP must stop where it is.
    pub fn ap_enter(&self) -> bool {
        self.0.compare_exchange(WAITING, ENTERED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    /// The boot CPU's claim when its wait expires. False means the AP entered
    /// in the meantime and is running.
    pub fn bsp_abandon(&self) -> bool {
        self.0.compare_exchange(WAITING, ABANDONED, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    pub fn entered(&self) -> bool {
        self.0.load(Ordering::Acquire) == ENTERED
    }
}

impl Default for BootClaim {
    fn default() -> Self {
        Self::new()
    }
}

/// Counter ticks for `ms` milliseconds at `hz`, or `fallback` when the rate is
/// not known (zero) or too low to express one millisecond.
pub const fn budget_ticks(ms: u64, hz: u64, fallback: u64) -> u64 {
    let ticks = (hz / 1000).saturating_mul(ms);
    if ticks == 0 {
        fallback
    } else {
        ticks
    }
}
