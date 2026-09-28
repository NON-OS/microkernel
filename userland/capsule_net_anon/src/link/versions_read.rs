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

//! Buffering the one cell whose circuit id is two bytes wide.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};
use nonos_tls::stream::Stream;

use super::session::LinkError;
use super::socket::Socket;
use super::timing::HANDSHAKE_MS;

pub(super) fn read_until_versions(
    stream: &mut Stream,
    socket: &mut Socket,
) -> Result<Vec<u8>, LinkError> {
    let mut buf = Vec::new();
    let deadline = mk_uptime_ms().saturating_add(HANDSHAKE_MS);
    loop {
        if buf.len() >= 5 {
            let want = 5 + u16::from_be_bytes([buf[3], buf[4]]) as usize;
            if buf.len() >= want {
                return Ok(buf);
            }
        }
        let more: Vec<u8> = stream.read(socket).map_err(|_| LinkError::Tls)?;
        if more.is_empty() {
            if mk_uptime_ms() >= deadline {
                return Err(LinkError::Protocol);
            }
            mk_yield();
            continue;
        }
        buf.extend_from_slice(&more);
    }
}
