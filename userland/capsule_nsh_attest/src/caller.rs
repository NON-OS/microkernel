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

//! The one-caller rule. nsh.attest answers nsh.front and no one else: a
//! quote binds the machine, so even among capsules it is handed to exactly
//! the measured front that relays it into a session it opened. The front's
//! pid is the one the kernel bound the `nsh.front` service name to at spawn,
//! resolved fresh each request so a restarted front is followed, not
//! impersonated.
//!
//! In a sealed release the front is named by its measurement, not only its
//! pid; the kernel's registry binds `nsh.front` to the measured capsule, so
//! this pid check rides on that binding. The development image has no second
//! Network capsule to race it.

use nonos_libc::mk_service_lookup;

/// The service name the front registers under, bound by the kernel at spawn.
const FRONT: &[u8] = b"nsh.front";

/// Whether `sender` is the measured nsh.front. A lookup that fails, or a pid
/// that does not match, is refused: nothing is quoted for an unknown caller.
pub fn is_front(sender: u32) -> bool {
    if sender == 0 {
        return false;
    }
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(FRONT.as_ptr(), FRONT.len(), &mut port, &mut pid);
    rc == 0 && pid != 0 && pid == sender
}
