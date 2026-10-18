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

//! Each CPU's suspend context: a static in the kernel image, which no
//! capsule is ever granted, and which never holds a key (asm/suspend.S).

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU64, Ordering};

use super::layout::{Q_OFFSET, SIZE, WORDS};
use crate::arch::aarch64::boot::stack::MAX_CPUS;

#[repr(C, align(16))]
pub(super) struct SuspendContext {
    pub(super) words: [u64; WORDS],
    pub(super) q: [u128; 32],
}

const _: () = assert!(core::mem::size_of::<SuspendContext>() == SIZE);
const _: () = assert!(core::mem::offset_of!(SuspendContext, q) == Q_OFFSET);

pub(super) struct Slots([UnsafeCell<SuspendContext>; MAX_CPUS]);

// SAFETY: CPU `i` alone touches slot `i`: Rust with IRQs masked from arming
// it until the call returns (powerdown.rs), the assembly and firmware only
// inside that call.
unsafe impl Sync for Slots {}

pub(super) static SLOTS: Slots =
    Slots([const { UnsafeCell::new(SuspendContext { words: [0; WORDS], q: [0; 32] }) }; MAX_CPUS]);

impl Slots {
    pub(super) fn get(&self, cpu: usize) -> Option<*mut SuspendContext> {
        self.0.get(cpu).map(UnsafeCell::get)
    }
}

/// Set by a resume that found no context its CPU armed (asm/suspend.S). From
/// then no CPU asks for a powerdown state: one came back somewhere it could
/// not return from.
#[no_mangle]
static AARCH64_RESUME_LOST: AtomicU64 = AtomicU64::new(0);

pub(super) fn resume_lost() -> bool {
    AARCH64_RESUME_LOST.load(Ordering::Acquire) != 0
}
