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

//! The two ways to complete a flight, authenticated or not.

use crate::flight::ClientFlight;

use super::complete::complete;
use super::types::ServerComplete;

/// Complete the handshake, requiring the certificate to chain to a trusted root
/// for `host`. `None` if the flight does not verify.
pub fn server_complete(
    client: &ClientFlight,
    bytes: &[u8],
    host: &[u8],
    now: u64,
) -> Option<ServerComplete> {
    complete(client, bytes, host, now, true)
}

/// Complete the handshake without walking the certificate chain.
///
pub fn server_complete_unauthenticated(
    client: &ClientFlight,
    bytes: &[u8],
) -> Option<ServerComplete> {
    complete(client, bytes, &[], 0, false)
}
