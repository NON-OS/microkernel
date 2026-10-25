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
//! One request over one connection.

extern crate alloc;

use alloc::vec::Vec;

use super::deadline::REPLY_MS;
use super::traits::{Io, SessionError};
use super::within::exchange_within;

/// Handshake with `host`, send `request`, and return the decrypted response,
/// the response within REPLY_MS of the request.
///
/// One request per connection. Keeping a session open would save a handshake,
/// but it means carrying the plaintext offset across requests, and drift
/// there hands the next caller somebody else's bytes.
pub fn exchange<S: Io>(
    io: &mut S,
    host: &str,
    request: &[u8],
    now: u64,
    limit: usize,
) -> Result<Vec<u8>, SessionError> {
    exchange_within(io, host, request, (now, limit), REPLY_MS)
}
