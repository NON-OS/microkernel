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

//! Verifying the flight and answering it in the same pass.

use alloc::vec::Vec;

use super::types::HandshakeState;
use crate::traffic_keys::TrafficKeys;

/// A verified handshake and the client's reply to it.
pub struct Answer {
    /// Application traffic keys, both directions.
    pub app: TrafficKeys,
    /// The client Finished record, then the request sealed at sequence zero.
    pub flight: Vec<u8>,
    /// The Certificate message body the server sent.
    pub certificates: Vec<u8>,
}

/// Why a handshake was not answered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The server stopped with this alert.
    Alert(u8),
    /// The chain, the CertificateVerify signature or the Finished MAC failed.
    Unverified,
    /// No whole Finished has arrived.
    Incomplete,
    /// The cipher would not seal the reply.
    Seal,
}

impl HandshakeState {
    /*
     * One walk yields both the application keys and the client's flight. The
     * request is sealed only after every check has passed, so a refused
     * certificate never carries the request to whoever answered.
     */
    /// Verify against `host` at `now` and seal `request` behind Finished.
    pub fn answer(&self, host: &[u8], now: u64, request: &[u8]) -> Result<Answer, Refusal> {
        if let Some(description) = self.alert {
            return Err(Refusal::Alert(description));
        }
        if self.end.is_none() {
            return Err(Refusal::Incomplete);
        }
        let done = self.verify(host, now, true).ok_or(Refusal::Unverified)?;
        super::reply(done, request)
    }
}
