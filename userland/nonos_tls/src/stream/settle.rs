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

//! Verifying the flight, answering Finished, and keeping the leftover bytes.

extern crate alloc;

use alloc::vec::Vec;

use crate::handshake_state::HandshakeState;
use crate::session::{Io, SessionError};

use super::types::Stream;

/*
 * The chain is the caller's to check, so it is not walked here; the
 * CertificateVerify signature and the Finished MAC are. Alerts have already
 * ended the flight before this point, so a failure here is the handshake's.
 */
pub(super) fn settle<S: Io>(
    io: &mut S,
    state: &HandshakeState,
    buf: Vec<u8>,
    end: usize,
) -> Result<Stream, SessionError> {
    let done = state.verify(&[], 0, false).ok_or(SessionError::Handshake)?;
    let record = crate::client_finished::client_finished(&done.handshake, &done.transcript_hash)
        .ok_or(SessionError::Handshake)?;
    io.write_all(&record)?;
    Ok(Stream::new(done.app, done.certificates, buf[end..].to_vec()))
}
