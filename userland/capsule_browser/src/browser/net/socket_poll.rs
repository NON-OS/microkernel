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

//! Whether a socket can be read or written, without touching its bytes.

use super::constants::{OP_POLL, SOCKETS_MAGIC};

const POLL_CALL_MS: u64 = 500;

/// The readiness bits of `handle`: bit0 when a read would return data, bit1
/// when a send would be taken, which for a connection being made is the
/// moment its handshake has finished. A connection that was refused or never
/// answered is simply never writable, so the caller keeps its own deadline.
pub fn socket_poll(sockets_port: u32, handle: u32) -> Result<u8, ()> {
    let mut rx = [0u8; 24];
    let n = super::call::call_t(
        sockets_port,
        SOCKETS_MAGIC,
        OP_POLL,
        &handle.to_le_bytes(),
        &mut rx,
        POLL_CALL_MS,
    )?;
    if n < 21 {
        return Err(());
    }
    Ok(rx[20])
}
