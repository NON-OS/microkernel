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

//! A one-way notice to the input router (NIRS v1): the frame, sent without
//! waiting. The router also calls the compositor, so a call each way could
//! stall both; it takes these from the compositor's pid only.

use alloc::vec::Vec;

use nonos_libc::{mk_ipc_send, mk_service_lookup};

const ROUTER: &[u8] = b"input_router";
const NIRS_MAGIC: u32 = 0x4E49_5253;
const NIRS_VERSION: u16 = 1;

/// Send op `op` with `body`; true when it went.
pub fn send(op: u16, body: &[u8]) -> bool {
    let (mut port, mut owner) = (0u32, 0u32);
    if mk_service_lookup(ROUTER.as_ptr(), ROUTER.len(), &mut port, &mut owner) < 0 || port == 0 {
        return false;
    }
    let Ok(len) = u32::try_from(body.len()) else { return false };
    let mut f = Vec::with_capacity(20 + body.len());
    f.extend_from_slice(&NIRS_MAGIC.to_le_bytes());
    f.extend_from_slice(&NIRS_VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0u8; 4]);
    f.extend_from_slice(&0u32.to_le_bytes());
    f.extend_from_slice(&len.to_le_bytes());
    f.extend_from_slice(body);
    mk_ipc_send(port as u64, f.as_ptr(), f.len()) >= 0
}
