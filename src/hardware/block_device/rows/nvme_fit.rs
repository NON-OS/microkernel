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

//! Whether an NVMe namespace can be addressed in 512-byte sectors.

use crate::hardware::nvme_capsule::lba_size_addressable;

/// Every caller addresses 512-byte sectors, and the NVMe client maps them
/// onto the namespace's own LBAs: a 4096-byte-LBA namespace is read whole
/// and written by read-modify-write at either unaligned end. Only an LBA
/// size the client cannot map (not a power of two, or larger than one
/// capsule command moves) is passed over by name rather than misaddressed.
pub(super) fn nvme_sectors_fit() -> bool {
    match crate::hardware::nvme_capsule::identify_namespace() {
        Ok(ns) if ns.lba_size != 0 && !lba_size_addressable(ns.lba_size) => {
            crate::log::warn!(
                "[BLOCK] NVMe namespace {} uses {}-byte blocks, which 512-byte sectors cannot map to; not used",
                ns.nsid,
                ns.lba_size
            );
            false
        }
        _ => true,
    }
}
