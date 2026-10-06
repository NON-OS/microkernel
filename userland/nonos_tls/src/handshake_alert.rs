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

//! Why the server stopped, once the handshake keys existed.

use crate::flight::ClientFlight;
use crate::handshake_state::{HandshakeState, Progress};

/// The alert description in the server's encrypted flight, or `None` if it sent
/// no alert. A caller holding a `HandshakeState` reads `alert()` instead and
/// pays for no second key agreement.
pub fn handshake_alert(client: &ClientFlight, bytes: &[u8]) -> Option<u8> {
    match HandshakeState::whole(client, bytes)? {
        (_, Progress::Alert(description)) => Some(description),
        _ => None,
    }
}
