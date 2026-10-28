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

//! The disk plan an update leaves in place, in the layout the kernel's
//! `parse_plan` reads (`src/fs/blockfs_volume/plan_types.rs`): it must name
//! a data volume, and every range it names, the volume and any import,
//! must lie in NONOS-DATA, the partition an update never writes.

use nonos_disk_map::{MIN_VOLUME_SECTORS, PLAN_MAGIC};

use super::why::NotKept;
use crate::boot_media::read::le64;
use crate::layout::Extent;
use crate::sink::SECTOR_SIZE;

/// As many imports as fit after the plan's 32-byte head, as the kernel
/// counts them (`MAX_IMPORTS`).
const MAX_IMPORTS: u64 = 30;

/// `s` is the plan's sector; `data` the data partition's extent.
pub fn plan_inside(s: &[u8], data: &Extent) -> Result<(), NotKept> {
    if s.len() < SECTOR_SIZE || s[..8] != PLAN_MAGIC {
        return Err(NotKept::Plan);
    }
    let inside = |first: u64, sectors: u64| {
        first >= data.first && data.end().checked_sub(first).is_some_and(|room| sectors <= room)
    };
    let (base, sectors, count) = (le64(s, 8), le64(s, 16), le64(s, 24));
    if sectors < MIN_VOLUME_SECTORS || !inside(base, sectors) || count > MAX_IMPORTS {
        return Err(NotKept::Plan);
    }
    for i in 0..count as usize {
        let (at, bytes) = (le64(s, 32 + 16 * i), le64(s, 40 + 16 * i));
        if bytes == 0 || !inside(at, bytes.div_ceil(SECTOR_SIZE as u64)) {
            return Err(NotKept::Plan);
        }
    }
    Ok(())
}

/// Whether a plan sector names a data volume, as an installed disk's does.
/// A live stick's plan names none: its volume's base and length are zero.
pub fn plan_names_volume(s: &[u8]) -> bool {
    s.len() >= 24 && s[..8] == PLAN_MAGIC && le64(s, 16) != 0
}
