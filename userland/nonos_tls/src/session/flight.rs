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

//! Collecting the server's handshake flight until it has finished.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{mk_uptime_ms, mk_yield};

use super::traits::{Io, SessionError};
use crate::flight::ClientFlight;
use crate::handshake_state::{HandshakeState, Progress};

/// How long a server may stay quiet in the middle of its flight. A duration
/// rather than a count of reads: an empty read costs microseconds.
const QUIET_MS: i64 = 4_000;

/*
 * The flight ends when a whole server Finished has been decrypted, not when
 * the socket goes quiet or some record count is reached: a server may send its
 * whole encrypted flight as one record, and a pause in the middle of a large
 * chain is not an ending. Returns the flight, the state that verified it, and
 * the offset where the server's application records begin.
 */
pub(super) fn read_flight<S: Io>(
    io: &mut S,
    client: &ClientFlight,
    limit: usize,
) -> Result<(Vec<u8>, HandshakeState, usize), SessionError> {
    let mut flight = Vec::new();
    let mut state: Option<HandshakeState> = None;
    let mut chunk = [0u8; 4096];
    let mut quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
    loop {
        let n = io.read(&mut chunk)?;
        if n == 0 {
            if mk_uptime_ms() >= quiet_until {
                return Err(SessionError::Handshake);
            }
            mk_yield();
            continue;
        }
        quiet_until = mk_uptime_ms().saturating_add(QUIET_MS);
        if flight.len() + n > limit {
            return Err(SessionError::TooLarge);
        }
        flight.extend_from_slice(&chunk[..n]);
        if state.is_none() {
            state = super::start::start(client, &flight)?;
        }
        let Some(hs) = state.as_mut() else { continue };
        match hs.advance(&flight) {
            Progress::Incomplete => {}
            Progress::Complete(end) => {
                return state.map(|hs| (flight, hs, end)).ok_or(SessionError::Handshake)
            }
            Progress::Alert(description) => return Err(SessionError::PeerAlert(description)),
            Progress::Broken => return Err(SessionError::Handshake),
        }
    }
}
