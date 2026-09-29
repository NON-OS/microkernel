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

/// Seal the client Finished and then `request`, the first application record,
/// for a flight that `done` says verified. Only a verified flight reaches
/// this: it is the step that lets the request leave.
pub(crate) fn reply(done: ServerComplete, request: &[u8]) -> Result<Answer, Refusal> {
    let th = &done.transcript_hash;
    let mut flight =
        crate::client_finished::client_finished(&done.handshake, th).ok_or(Refusal::Seal)?;
    let a = &done.app;
    let record = crate::record_seal::seal(a.suite, &a.client_key, &a.client_iv, 0, 23, request)
        .ok_or(Refusal::Seal)?;
    flight.extend_from_slice(&record);
    Ok(Answer { app: done.app, flight, certificates: done.certificates })
}
