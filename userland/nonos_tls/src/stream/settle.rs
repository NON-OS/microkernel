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

//! Completing the flight, its chain checked when a host is named, answering
//! Finished, and keeping the leftover bytes.

extern crate alloc;

use alloc::vec::Vec;

use crate::flight::ClientFlight;
use crate::session::{Io, SessionError};

use super::types::Stream;

pub(super) fn settle<S: Io>(
    io: &mut S,
    client: &ClientFlight,
    buf: Vec<u8>,
    end: usize,
    peer: Option<(&[u8], u64)>,
) -> Result<Stream, SessionError> {
    let flight = &buf[..end];
    /*
     * Handshake is the fault only when the peer did not say what was wrong.
     * With a host named, a chain that does not verify for it is Certificate.
     */
    let (done, quiet) = match peer {
        Some((host, now)) => {
            (crate::server_complete(client, flight, host, now), SessionError::Certificate)
        }
        None => (crate::server_complete_unauthenticated(client, flight), SessionError::Handshake),
    };
    let done = done.ok_or_else(|| crate::handshake_fault(client, flight, quiet))?;
    let record = crate::client_finished::client_finished(&done.handshake, &done.transcript)
        .ok_or(SessionError::Handshake)?;
    io.write_all(&record)?;
    Ok(Stream::new(done.app, done.certificates, buf[end..].to_vec()))
}
