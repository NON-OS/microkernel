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

//! Each CPU's keys, in its per-CPU block (smp::percpu::PerCpuData.pac),
//! which TPIDR_EL1 names on every CPU and which the exception vectors read
//! at fixed offsets (asm/percpu.h): words 0-1 the IA key of the process this
//! CPU next enters EL0 with, 2-3 the kernel's, 4 non-zero once this CPU
//! switches keys. TPIDR_EL1 has that one meaning; nothing here writes it.
//! EL0 cannot read it.

use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::smp::percpu::{by_base, get, PerCpuData};

const USER: usize = 0;
const KERNEL: usize = 2;
const ON: usize = 4;

/// Fill CPU `cpu`'s block with the kernel key and turn key switching on
/// for it. False for an index past the blocks.
pub(super) fn bind(cpu: usize, kernel: [u64; 2]) -> bool {
    let Some(block) = get(cpu) else { return false };
    for (word, value) in block.pac[KERNEL..KERNEL + 2].iter().zip(kernel) {
        word.store(value, Ordering::Relaxed);
    }
    // After the key: the vectors take the key once they see this word.
    block.pac[ON].store(1, Ordering::Release);
    true
}

/// The block TPIDR_EL1 names on this CPU, if it names one.
fn here() -> Option<&'static PerCpuData> {
    let at: u64;
    // SAFETY: reading TPIDR_EL1 at EL1 has no effect.
    unsafe { asm!("mrs {}, tpidr_el1", out(reg) at, options(nomem, nostack, preserves_flags)) };
    by_base(at)
}

/// The key this CPU installs on its next entry to EL0 through
/// `aarch64_enter_user` or `aarch64_resume_user`.
pub(super) fn set_user(key: [u64; 2]) {
    let Some(block) = here() else { return };
    for (word, value) in block.pac[USER..USER + 2].iter().zip(key) {
        word.store(value, Ordering::Relaxed);
    }
}
