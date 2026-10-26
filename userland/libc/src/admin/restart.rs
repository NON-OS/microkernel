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

//! A restart asked of capsule_power's `power` service, op 2: the installers'
//! last step, for which they hold no Admin of their own (S-14). The service
//! lets the shell and the two installers ask; it wipes as a power-off does.

use crate::{mk_ipc_call_timeout, mk_service_lookup};

/// Magic 0x504F_5752, version 1, op 2 (reboot), no payload: capsule_power's header.
const REQUEST: [u8; 20] = [b'R', b'W', b'O', b'P', 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const STATUS_AT: usize = 20;

/// Ask for the wiped restart. One that runs never returns; a refusal comes
/// back as its status (EPERM for a caller it does not admit), a failed call
/// as its errno, and ENODEV when no power service is running.
pub fn power_restart() -> i64 {
    let (name, mut port, mut pid) = (b"power", 0u32, 0u32);
    if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) < 0 || port == 0 {
        return -19;
    }
    let mut reply = [0u8; STATUS_AT + 4];
    let rc = mk_ipc_call_timeout(
        u64::from(port),
        REQUEST.as_ptr(),
        REQUEST.len(),
        reply.as_mut_ptr(),
        reply.len(),
        10_000,
    );
    if rc < 0 {
        return rc;
    }
    if (rc as usize) < reply.len() {
        return -71;
    }
    i64::from(i32::from_le_bytes([reply[20], reply[21], reply[22], reply[23]]))
}
