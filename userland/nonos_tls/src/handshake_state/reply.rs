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

//! The client's half of the last handshake flight, once it is earned.

use super::answer::{Answer, Refusal};
use crate::server_complete::ServerComplete;

/// The largest plaintext one record may carry.
const RECORD_MAX: usize = 1 << 14;

/// Seal the client Finished and then `request`, in application records of at
/// most 2^14 bytes each (RFC 8446, 5.1), numbered from zero, for a flight that
/// `done` says verified. Only a verified flight reaches this: it is the step
/// that lets the request leave. One record used to carry any request, so one
/// over 16 KiB was a record the server refused, and one over 64 KiB wrote a
/// length that had wrapped.
pub(crate) fn reply(done: ServerComplete, request: &[u8]) -> Result<Answer, Refusal> {
    // The empty Certificate first when the server asked for one, as the
    // session path does: a server that asked ends on a bare Finished.
    let mut flight = crate::client_finished::client_reply(&done).ok_or(Refusal::Seal)?;
    let a = &done.app;
    let pieces: alloc::vec::Vec<&[u8]> =
        if request.is_empty() { alloc::vec![request] } else { request.chunks(RECORD_MAX).collect() };
    for (seq, piece) in pieces.iter().enumerate() {
        let record = crate::record_seal::seal(a.suite, &a.client_key, &a.client_iv, seq as u64, 23, piece)
            .ok_or(Refusal::Seal)?;
        flight.extend_from_slice(&record);
    }
    Ok(Answer { app: done.app, flight, certificates: done.certificates })
}
