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

//! Reading until ServerHello, and what else can arrive before it.

extern crate alloc;

use alloc::vec::Vec;

use crate::flight::ClientFlight;
use crate::handshake_state::HandshakeState;
use crate::session::{Io, SessionError};

use super::gather::gather;

/*
 * A retry and an alert in the clear are both answers already in the buffer,
 * so they end the wait instead of reading on for a ServerHello that is not
 * coming; the session's start says which is which.
 */
pub(super) fn handshake_keys<S: Io>(
    io: &mut S,
    client: &ClientFlight,
    buf: &mut Vec<u8>,
) -> Result<HandshakeState, SessionError> {
    loop {
        if let Some(state) = crate::session::start::start(client, buf)? {
            return Ok(state);
        }
        gather(io, buf)?;
    }
}
