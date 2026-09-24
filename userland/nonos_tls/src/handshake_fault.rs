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

//! Telling a refusal from a failure.

use crate::flight::ClientFlight;
use crate::session::SessionError;

/*
 * One function, because the session and the stream both need this answer and
 * two copies of it would drift the way the two TLS trees did.
 */
pub fn handshake_fault(client: &ClientFlight, flight: &[u8], quiet: SessionError) -> SessionError {
    match crate::handshake_alert(client, flight) {
        Some(description) => SessionError::PeerAlert(description),
        None => quiet,
    }
}
