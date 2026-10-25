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
//! the compositor's poll reads as the display gone; it is told first, one
//! send of OUTPUT_NOTICE into its inbox, so it moves to another display at
//! once, and a send refused changes nothing. Any other frame from
//! sender 0 is dropped, and sender 0 is never answered (`turn`).

use nonos_gfx_wire::notice::{notice, LEAVING};
use nonos_libc::{mk_exit, mk_ipc_send_to_pid, EXIT_ABSENT};

/// A DGON named this driver's device (`turn`); `owner` is the compositor
/// that asked for its primaries, if one has.
pub fn leave(owner: Option<u32>) -> ! {
    if let Some(pid) = owner {
        let n = notice(LEAVING);
        let _ = mk_ipc_send_to_pid(pid, n.as_ptr(), n.len());
    }
    mk_exit(EXIT_ABSENT)
}
