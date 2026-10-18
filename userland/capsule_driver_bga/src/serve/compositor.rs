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

//! Who the compositor is: the process its endpoint is registered to. The
//! kernel registers the name at the compositor's spawn, a respawned one's
//! too, and no MkServiceRegister can take it. Nor can a capsule loaded at
//! runtime, even while the name is free between a death and a respawn: the
//! kernel spawns no publisher's capsule under a name the image's own
//! capsules declare, and `compositor` is one of them.

use nonos_libc::mk_service_lookup;

/// The name the compositor's endpoint holds (its Capsule.mk handle).
pub const COMPOSITOR_SERVICE: &[u8] = b"compositor";

/// The compositor's pid, or `None` while no process holds the name.
pub fn compositor_pid() -> Option<u32> {
    let (name, mut pid) = (COMPOSITOR_SERVICE, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), core::ptr::null_mut(), &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}
