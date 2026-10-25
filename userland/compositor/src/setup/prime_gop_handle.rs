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

//! The compositor's handle on the GOP surface it registers. A sid is
//! a slot, not a handle, and MkSurfaceAttach and MkSurfacePresent take the
//! handle. A share that names the owner's own process hands that back and
//! records no receiver. The compositor names itself by the endpoint the
//! kernel registers for it at spawn, the name the display drivers hand their
//! surfaces to, so it needs no MkGetPid, and no CoreExec for one. Pure:
//! compositor_proofs mounts it.

/// The name the kernel registers the compositor's endpoint under.
pub const COMPOSITOR_SERVICE: &[u8] = b"compositor";

/// `sid`'s handle: shared, with no write right, with the process `lookup`
/// finds under the compositor's own name, which is this one.
pub fn own_handle(
    sid: u64,
    lookup: impl FnOnce(&[u8]) -> Option<u32>,
    share: impl FnOnce(u64, u32, u64) -> i64,
) -> Result<u64, &'static str> {
    let own = lookup(COMPOSITOR_SERVICE).ok_or("gop: own endpoint not registered")?;
    let handle = share(sid, own, 0);
    if handle <= 0 {
        return Err("gop: surface handle refused");
    }
    Ok(handle as u64)
}
