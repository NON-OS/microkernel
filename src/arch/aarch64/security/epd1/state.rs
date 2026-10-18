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

//! The one state word `step` moves, and the counts the boot line reports.

use core::arch::asm;
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

use super::step;
use crate::arch::aarch64::mmu::trampoline::{self, geometry};

static STATE: AtomicU8 = AtomicU8::new(step::IDLE);
pub(super) static EXPOSED: AtomicU32 = AtomicU32::new(0);
pub(super) static SWITCHED: AtomicU32 = AtomicU32::new(0);

fn advance(f: fn(u8) -> (u8, bool)) -> bool {
    let mut allowed = false;
    let _ = STATE.fetch_update(Ordering::AcqRel, Ordering::Acquire, |now| {
        let (next, yes) = f(now);
        allowed = yes;
        Some(next)
    });
    allowed
}

pub(super) fn built() {
    advance(|now| (step::built(now), false));
}

/// Point this CPU's vectors at the trampoline if `step` allows it. From
/// then on every exit to EL0 on this CPU goes through its exit, which reads
/// VBAR_EL1 to know (asm/epd1.h).
pub(super) fn switch_here() {
    if !advance(step::exposed) {
        return;
    }
    // SAFETY: the trampoline is in the kernel's root and, from the state
    // now ON, in every user root this CPU can load; its slots land on the
    // same handlers the kernel's own table does.
    unsafe { asm!("msr vbar_el1, {}", "isb", in(reg) geometry::BASE, options(nostack)) };
    SWITCHED.fetch_add(1, Ordering::AcqRel);
}

/// The slot and word a user root being made now carries, if any.
pub fn user_root_word() -> Option<(usize, u64)> {
    if advance(step::root) {
        trampoline::root_word()
    } else {
        None
    }
}
