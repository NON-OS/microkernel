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


//! Waiting for the server's next bytes, bounded in time and in size.

extern crate alloc;

use alloc::vec::Vec;

use nonos_libc::{mk_uptime_ms, mk_yield};
use nonos_tls::Io;

use super::constants::{FLIGHT_MAX, QUIET_MS};
use super::error::Tls12Error;

/// Append what the server sends next to `buf`. A server that sends nothing
/// for QUIET_MS, or whose bytes would take `buf` past FLIGHT_MAX, ends the
/// handshake.
pub fn gather<S: Io>(io: &mut S, buf: &mut Vec<u8>) -> Result<(), Tls12Error> {
    let mut chunk = [0u8; 4096];
    let deadline = mk_uptime_ms().saturating_add(QUIET_MS);
    loop {
        let n = io.read(&mut chunk).map_err(|_| Tls12Error::Io)?;
        if n == 0 {
            if mk_uptime_ms() >= deadline {
                return Err(Tls12Error::Quiet);
            }
            mk_yield();
            continue;
        }
        if buf.len().saturating_add(n) > FLIGHT_MAX {
            return Err(Tls12Error::TooLarge);
        }
        buf.extend_from_slice(&chunk[..n.min(chunk.len())]);
        return Ok(());
    }
}
