// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::ram;

/// The sectors the volume may allocate in: the window this boot's plan
/// opened, which the cryptoblock layer refuses to go past. A volume was
/// formatted to its plan's window; a later plan that gives it more lets it
/// grow into the room, and one that gives it less must not be allocated past.
pub fn alloc_limit(mount: &BlockFsMount) -> u64 {
    crate::fs::cryptoblock::window_sectors().unwrap_or(mount.superblock.sectors)
}

pub fn alloc_block(mount: &mut BlockFsMount) -> Result<u64, BlockFsError> {
    let lba = mount.superblock.free_lba;
    if lba >= alloc_limit(mount) {
        return Err(BlockFsError::OutOfSpace);
    }
    /* A volume in RAM stops growing while the machine is short of memory. */
    if lba % ram::ROOM_EVERY == 0 && ram::on() && !ram::room() {
        return Err(BlockFsError::OutOfSpace);
    }
    mount.superblock.free_lba = lba + 1;
    Ok(lba)
}
