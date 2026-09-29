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

//! Opening a session whose peer the caller will authenticate itself.

extern crate alloc;

use alloc::vec::Vec;

use crate::handshake_state::Progress;
use crate::session::{Io, SessionError};

use super::gather::gather;
use super::handshake_keys::handshake_keys;
use super::settle::settle;
use super::types::Stream;

/// Handshake with a peer whose certificate is not expected to chain to a public
/// root, and return a session that stays open.
pub fn connect_unauthenticated<S: Io>(io: &mut S, sni: &[u8]) -> Result<Stream, SessionError> {
    let client = crate::client_flight(sni).ok_or(SessionError::Init)?;
    io.write_all(&client.record)?;

    let mut buf: Vec<u8> = Vec::new();
    /*
     * Keyed once, and each record decrypted once as it arrives: the state
     * that finds the Finished is the one that verifies it, so the key
     * agreement is not repeated at the end.
     */
    let mut state = handshake_keys(io, &client, &mut buf)?;
    loop {
        match state.advance(&buf) {
            Progress::Broken => return Err(SessionError::Handshake),
            Progress::Alert(description) => return Err(SessionError::PeerAlert(description)),
            Progress::Complete(end) => return settle(io, &state, buf, end),
            Progress::Incomplete => gather(io, &mut buf)?,
        }
    }
}
