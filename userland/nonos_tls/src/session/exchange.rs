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

use super::flight::read_flight;
use super::response::read_response;
use super::traits::{Io, SessionError};
use crate::app_reader::AppReader;
use crate::handshake_state::Refusal;

/// Bound on the handshake flight, so a server cannot make a caller allocate
/// without limit before anything has been verified.
const MAX_FLIGHT: usize = 128 * 1024;

/// Handshake with `host`, send `request`, and return the decrypted response.
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
    let cf = crate::client_flight(host.as_bytes()).ok_or(SessionError::Init)?;
    io.write_all(&cf.record)?;

    let (flight, state, end) = read_flight(io, &cf, MAX_FLIGHT)?;

    /*
     * The chain, the CertificateVerify and Finished are checked here, once,
     * and the request is sealed only if all of them passed: sending it anyway
     * would hand the payload to whoever answered.
     */
    let answer = state.answer(host.as_bytes(), now, request).map_err(|refusal| match refusal {
        Refusal::Alert(description) => SessionError::PeerAlert(description),
        Refusal::Unverified => SessionError::Certificate,
        Refusal::Incomplete | Refusal::Seal => SessionError::Handshake,
    })?;
    io.write_all(&answer.flight)?;

    /* Whatever followed the server's Finished in the same read is response. */
    let buf = read_response(io, flight[end..].to_vec(), limit)?;
    let mut reader = AppReader::new();
    reader.feed(&answer.app, &buf);
    Ok(reader.into_plaintext())
}
