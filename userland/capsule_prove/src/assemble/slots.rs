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

//! This boot's two slots as the witness takes them, from the `MkBootSlots`
//! record through libc's own reader. A v3 path is a 9-byte header, then one
//! 32-byte sibling per level, then the directions: bit k of byte k/8 set when
//! the running node is the right child at level k.

use alloc::vec::Vec;

use nonos_device_attest::{words_of, Path, Slot, BOOT_DEPTH, KERNEL_DEPTH};

use super::error::Refusal;
use crate::abi::{parse_boot_slots, BootSlot, PATH_LEN};

const HEADER: usize = 9;
const DIRS: usize = HEADER + 32 * BOOT_DEPTH;

/* The record's path is exactly the circuit's depth, for both trees. */
const _: () = assert!(PATH_LEN == DIRS + BOOT_DEPTH.div_ceil(8));
const _: () = assert!(KERNEL_DEPTH == BOOT_DEPTH);

pub struct Slots {
    pub bootloader: Slot,
    pub kernel: Slot,
    /// The roots the two paths fold to, as the kernel worked them out.
    pub boot_root: [u8; 32],
    pub kernel_root: [u8; 32],
}

pub fn slots(record: &[u8]) -> Result<Slots, Refusal> {
    let s = parse_boot_slots(record).ok_or(Refusal::Slots)?;
    Ok(Slots {
        bootloader: slot(&s.bootloader).ok_or(Refusal::Slots)?,
        kernel: slot(&s.kernel).ok_or(Refusal::Slots)?,
        boot_root: s.bootloader.root,
        kernel_root: s.kernel.root,
    })
}

fn slot(s: &BootSlot) -> Option<Slot> {
    let mut siblings = Vec::with_capacity(BOOT_DEPTH);
    for k in 0..BOOT_DEPTH {
        let at = HEADER + 32 * k;
        siblings.push(words_of(s.path.get(at..at + 32)?.try_into().ok()?));
    }
    let dirs = s.path.get(DIRS..)?;
    let right: Option<Vec<bool>> =
        (0..BOOT_DEPTH).map(|k| dirs.get(k / 8).map(|d| d >> (k % 8) & 1 == 1)).collect();
    Some(Slot { digest: s.digest, path: Path { siblings, right: right? } })
}
