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

//! Keying the handshake once the ServerHello has arrived, as a session error.

use super::traits::SessionError;
use crate::flight::ClientFlight;
use crate::handshake_state::{HandshakeState, Start};

/// `Ok(None)` while the ServerHello is still arriving; the keyed state once it
/// is whole; the reason when the server's first record ends the handshake.
pub(crate) fn start(
    client: &ClientFlight,
    flight: &[u8],
) -> Result<Option<HandshakeState>, SessionError> {
    match HandshakeState::begin(client, flight) {
        Start::Waiting => Ok(None),
        Start::Ready(state) => Ok(Some(*state)),
        Start::Alert(description) => Err(SessionError::PeerAlert(description)),
        /*
         * This client sends a share for every group it offers (x25519 and
         * secp256r1), so a retry asks for one it lacks: say the server asked.
         */
        Start::Retry => Err(SessionError::RetryUnsupported),
        Start::Unusable => Err(SessionError::Handshake),
    }
}
