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

//! The kernel's word that the adapter has left (DGON, from sender 0). By the
//! time it is read the device is out of the broker's table, its grants are
//! revoked and its MMIO is fenced, so nothing is touched again: the driver
//! exits at once without running a destructor, each of which would unmap a
//! grant, release the claim or write a register, and the kernel takes back
//! what the process held. The service leaves the registry with it, which
//! the compositor's poll reads as the display gone. Any other frame from
//! sender 0 is dropped, and sender 0 is never answered.

use nonos_libc::{dgon_parse, mk_exit, EXIT_ABSENT};

/// A frame from sender 0: ends the driver when it is a DGON for `device_id`.
pub fn from_kernel(msg: &[u8], device_id: u64) {
    if dgon_parse(msg, 0).is_some_and(|g| g.device_id == device_id) {
        mk_exit(EXIT_ABSENT);
    }
}
