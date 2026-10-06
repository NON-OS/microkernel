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

//! Driving a whole flight to Finished and deriving the application keys.

use crate::flight::ClientFlight;
use crate::handshake_state::{HandshakeState, Progress};

use super::types::ServerComplete;

/*
 * The same walk and the same checks as a caller holding the handshake state
 * across reads; this is that path for a caller that has the flight in one
 * buffer. It keys, decrypts and verifies once per call.
 */
pub(super) fn complete(
    client: &ClientFlight,
    bytes: &[u8],
    host: &[u8],
    now: u64,
    require_chain: bool,
) -> Option<ServerComplete> {
    let (state, progress) = HandshakeState::whole(client, bytes)?;
    if !matches!(progress, Progress::Complete(_)) {
        return None;
    }
    state.verify(host, now, require_chain)
}
