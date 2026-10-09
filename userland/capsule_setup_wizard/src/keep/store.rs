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

//! Whether this boot has a store to keep anything in.

use nonos_app_skeleton::clients::vfs;

/*
 * The store status codes that mean vfs never read a NONOS disk: no block
 * service, a transport fault, a short reply, or a read the device refused
 * (capsule_vfs/src/blk/status.rs). Any other code was read from a disk.
 */
const NO_DISK: [u32; 4] = [1, 2, 3, 7];

/// True once vfs has settled the store from a NONOS disk. A store with an
/// entry refused (status 9) is still a disk: one bad entry said the whole
/// boot had none, closed every Qwen tier, and kept nothing setup chose.
pub fn store_ready() -> bool {
    vfs::store_settled() == Ok(true) && vfs::store_status().is_ok_and(|s| !NO_DISK.contains(&s))
}

/// True while vfs is still finding the disk: the store has not settled. On a
/// slow machine setup can start before it does, so this is not yet a boot
/// without a disk.
pub fn store_pending() -> bool {
    vfs::store_settled() == Ok(false)
}
