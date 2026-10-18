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

//! The capsule's own pid, read back from the registry entry the kernel
//! made when it spawned the capsule under its service name. mk_getpid would
//! need CoreExec, which a class driver above xHCI has no other use for;
//! MkServiceLookup needs only IPC.

use nonos_libc::mk_service_lookup;

/// 0 when the name is not registered: no sender is 0, so nothing is then
/// taken for the capsule's own message.
pub fn own_pid(service: &str) -> u32 {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc == 0 {
        pid
    } else {
        0
    }
}
