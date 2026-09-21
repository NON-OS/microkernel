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
use crate::session::{Io, SessionError};
use crate::traffic_keys::TrafficKeys;

use super::gather::gather;

pub(super) fn handshake_keys<S: Io>(
    io: &mut S,
    client: &ClientFlight,
    buf: &mut Vec<u8>,
) -> Result<(TrafficKeys, usize), SessionError> {
    loop {
        if let Some(ctx) = crate::server_keys::server_keys(client, buf) {
            return Ok((ctx.keys, ctx.used));
        }
        /*
         * Checked before reading on, because reading on waits for a message the server
         * has already sent.
         */
        if crate::hello_retry::in_buffer(buf) {
            return Err(SessionError::RetryUnsupported);
        }
        /*
         * Not a handshake record, so the search for a ServerHello skipped it and the
         * caller waited out its timeout with the answer already in the buffer.
         */
        if let Some(description) = crate::alert::description_in_record(buf) {
            return Err(SessionError::PeerAlert(description));
        }
        gather(io, buf)?;
    }
}
