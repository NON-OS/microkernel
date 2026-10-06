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

/*
 * Whether a disk is the boot media by the loader's record alone, with no
 * loader bytes to fall back on.
 */

use nonos_disk::{is_boot_media, BootEvidence, BootPartition};

use crate::mem_disk::MemDisk;

pub fn named(disk: &mut MemDisk, partition: BootPartition) -> bool {
    let ev = BootEvidence { partition: Some(partition), loader_size: 0, loader_head: &[] };
    is_boot_media(disk, &ev)
}
