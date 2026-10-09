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

//! What a directory fetch reports about how far it got.

use alloc::vec::Vec;
use nonos_tls::SessionError;

/// The body length on success, or the stage the session gave up at.
pub(super) fn stage(raw: &Result<Vec<u8>, SessionError>) -> u64 {
    match raw {
        Ok(body) => body.len() as u64,
        Err(SessionError::Init) => 1,
        Err(SessionError::Io) => 2,
        Err(SessionError::Handshake) => 3,
        Err(SessionError::Certificate) => 4,
        Err(SessionError::TooLarge) => 5,
        Err(SessionError::RetryUnsupported) => 6,
        /*
         * The peer's own description, offset past the stage numbers above so
         * a reader can tell an alert from a stage this client gave up at.
         */
        Err(SessionError::PeerAlert(description)) => 100 + *description as u64,
    }
}

/// Whether the response line says 200. Anything else is not a node list.
pub(super) fn status_ok(resp: &[u8]) -> bool {
    resp.len() > 12 && resp.starts_with(b"HTTP/1.1 200")
}
