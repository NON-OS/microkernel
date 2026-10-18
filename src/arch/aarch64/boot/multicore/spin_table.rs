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

//! Releasing a CPU that firmware parked on a spin table, as the Raspberry
//! Pi 4's does: no PSCI, just a 64-bit location per CPU that it polls with
//! WFE until a non-zero entry address appears (Devicetree Specification
//! v0.4, 5.4.2).
//!
//! The parked CPU reads with its MMU and caches off, so everything it will
//! read, the release word and the stack table, is written to the point of
//! coherency first (Arm ARM, DDI 0487, DC CVAC and the cache-disabled
//! behaviour of data accesses), and SEV follows a DSB so the wake cannot
//! overtake the write it announces.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::aarch64::mmu::pointer::ram;
use crate::arch::cache_flush::flush_and_fence;

/// As many CPUs as `spin_table_start.S` searches for; it hard-codes the 64.
const SLOTS: usize = 64;

/// (affinity, stack top) pairs, read by `_aarch64_spin_table_start`.
#[no_mangle]
static AARCH64_SPIN_STACKS: [AtomicU64; 2 * SLOTS] = [const { AtomicU64::new(0) }; 2 * SLOTS];

/// Give the CPU with `affinity` its stack, in the table its entry stub reads.
pub(super) fn assign_stack(
    slot: usize,
    affinity: u64,
    stack_top: u64,
) -> Result<(), &'static [u8]> {
    let pair = AARCH64_SPIN_STACKS.get(2 * slot..2 * slot + 2);
    let pair = pair.ok_or(&b"no spin-table slot"[..])?;
    pair[0].store(affinity, Ordering::Relaxed);
    pair[1].store(stack_top, Ordering::Release);
    flush_and_fence(pair.as_ptr() as u64, 16);
    Ok(())
}

/// Write `entry` to the release word at `release` and wake the parked CPUs.
///
/// The word has to be RAM the kernel maps, and 8-byte aligned, or the tree
/// is wrong and the write would land on something else.
pub(super) fn release(info: &BootInfo, release: u64, entry: u64) -> Result<(), &'static [u8]> {
    let in_ram = info.memory_map().iter().any(|r| release >= r.base && release - r.base < r.size);
    if release % 8 != 0 || !in_ram {
        return Err(b"cpu-release-addr is not aligned RAM");
    }
    let word = ram::<u64>(release).ok_or(&b"cpu-release-addr is past the direct map"[..])?;
    // SAFETY: the address is aligned RAM inside the kernel's direct map, and
    // firmware gave this word to the parked CPU, so no Rust object lives there.
    unsafe { core::ptr::write_volatile(word, entry) };
    flush_and_fence(word as u64, 8);
    // SAFETY: SEV only signals the event register of every CPU.
    unsafe { core::arch::asm!("sev", options(nomem, nostack, preserves_flags)) };
    Ok(())
}
