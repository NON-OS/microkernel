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

//! Waiting for more bytes during a handshake, with a bound on both time and size.

use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::session::{Io, SessionError};

use super::limits::FLIGHT_MAX;

const QUIET_MS: i64 = 8_000;

pub(super) fn gather<S: Io>(io: &mut S, buf: &mut Vec<u8>) -> Result<(), SessionError> {
    let mut chunk = [0u8; 4096];
    let deadline = mk_uptime_ms().saturating_add(QUIET_MS);
    loop {
        let n = io.read(&mut chunk)?;
        if n == 0 {
            if mk_uptime_ms() >= deadline {
                return Err(SessionError::Handshake);
            }
            mk_yield();
            continue;
        }
        if buf.len() + n > FLIGHT_MAX {
            return Err(SessionError::TooLarge);
        }
        buf.extend_from_slice(&chunk[..n]);
        return Ok(());
    }
}
