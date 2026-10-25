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

//! The per-CPU "this return is a resume" flag. A saved context returns twice,
//! once when it is taken and once when it is put back; the restore raises the
//! flag so the caller can tell the two apart. It is per CPU because two harts
//! can be mid-switch at once.

use core::sync::atomic::{AtomicBool, Ordering};

const MAX_CPUS: usize = 256;
const INIT_FALSE: AtomicBool = AtomicBool::new(false);

pub(super) static CONTEXT_JUST_RESTORED: [AtomicBool; MAX_CPUS] = [INIT_FALSE; MAX_CPUS];

/// This CPU's slot. A hart id past the table is a machine this kernel does
/// not size for, and indexing stops the kernel there rather than share a slot.
#[inline]
pub(super) fn current_cpu_index() -> usize {
    crate::arch::riscv64::cpu::cpu_id()
}

/// Mark the current CPU as having just come back from a saved context.
pub(super) fn set_restored_flag() {
    CONTEXT_JUST_RESTORED[current_cpu_index()].store(true, Ordering::SeqCst);
}
