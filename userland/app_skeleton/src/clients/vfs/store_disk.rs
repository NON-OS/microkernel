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

//! Whether vfs read a NONOS disk on this boot, from its store status.
//!
//! The status latches the first fault of the store's load. Only four codes
//! mean no disk was read at all: no block service, a transport fault, a
//! short reply, or a read the device refused (capsule_vfs blk/status.rs:
//! 1, 2, 3, 7). Any other code came from a disk that was there. The commonest
//! is 9, which vfs also latches when the store loaded with one damaged entry
//! left out; reading every nonzero code as "no disk" made one bad entry close
//! saved Wi-Fi networks and every store-backed feature on a disk that worked.
//! A disk whose store is genuinely broken still fails the write itself, and
//! that failure is reported where it happens.

/// The store status codes that mean vfs never read a NONOS disk.
pub const NO_DISK: [u32; 4] = [1, 2, 3, 7];

/// Whether a store status says a NONOS disk was read.
pub fn store_was_read(status: u32) -> bool {
    !NO_DISK.contains(&status)
}

/// Whether vfs has settled its store from a NONOS disk on this boot.
pub fn store_has_disk() -> bool {
    super::store_settled() == Ok(true) && super::store_status().is_ok_and(store_was_read)
}
