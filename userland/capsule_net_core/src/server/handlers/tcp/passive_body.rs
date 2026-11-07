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

//! The bodies of LISTEN and ACCEPT as net.core decodes them: LISTEN carries
//! the port to listen on, ACCEPT the listener's handle, both little endian.
//! Bytes past them are not read.

use crate::protocol::tcp::{E_BAD_ADDR, E_BAD_LEN};

/// The port a LISTEN names; port 0 is no port.
pub fn listen_port(body: &[u8]) -> Result<u16, u16> {
    let port = match body {
        [lo, hi, ..] => u16::from_le_bytes([*lo, *hi]),
        _ => return Err(E_BAD_LEN),
    };
    if port == 0 {
        return Err(E_BAD_ADDR);
    }
    Ok(port)
}

/// The listener handle an ACCEPT names.
pub fn accept_handle(body: &[u8]) -> Result<u32, u16> {
    match body {
        [a, b, c, d, ..] => Ok(u32::from_le_bytes([*a, *b, *c, *d])),
        _ => Err(E_BAD_LEN),
    }
}
