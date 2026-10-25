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
//! Who a primary is shared with: the compositor the kernel names now, and
//! any later one the wire's owner rule lets attach.

use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::Errno;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
use nonos_libc::{mk_service_lookup, mk_surface_share};

/// The compositor's pid, by the endpoint the kernel registers for it.
pub fn compositor_pid() -> Option<u32> {
    let name = b"compositor";
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

/// Name `pid`, the compositor, a writable receiver of `sid` as well. Rights
/// die with the pid that held them (K20), so a restarted compositor needs its
/// own; naming one twice is no error.
pub fn reshare(sid: u64, pid: u32) -> Result<(), Errno> {
    // The kernel holds a display driver to the compositor; the pool keeps
    // the same rule itself, so no other caller is ever named a receiver.
    if compositor_pid() != Some(pid) {
        return Err(EINVAL);
    }
    (mk_surface_share(sid, pid, SURFACE_SHARE_WRITE) > 0).then_some(()).ok_or(EINVAL)
}
