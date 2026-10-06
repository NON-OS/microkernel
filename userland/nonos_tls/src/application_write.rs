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

use alloc::vec::Vec;

use super::flight::ClientFlight;
use super::handshake_state::HandshakeState;

/// Verify a whole flight against `host` and return the client Finished record
/// followed by `body` sealed as the first application record. `None` if the
/// flight is incomplete, the server sent an alert, or any check fails.
pub fn application_write(
    client: &ClientFlight,
    bytes: &[u8],
    body: &[u8],
    host: &[u8],
    now: u64,
) -> Option<Vec<u8>> {
    let (state, _) = HandshakeState::whole(client, bytes)?;
    state.answer(host, now, body).ok().map(|answer| answer.flight)
}
