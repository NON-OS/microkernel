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

//! The server's flight taken as far as its bytes allow, then judged once:
//! the chain, CertificateVerify and Finished, and the request sealed only
//! if all of them held.

use alloc::vec::Vec;

use nonos_tls::{HandshakeState, Progress, Start};

use super::super::types::{After, Exchange, Phase};
use super::{FAILED, REFUSED, RETRY};

impl Exchange {
    /// Take the flight as far as the bytes so far allow.
    pub(in crate::mixnet::exchange) fn advance(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let Some(tls) = self.tls.as_mut() else {
            return Err(FAILED);
        };
        if tls.handshake.is_none() {
            match HandshakeState::begin(&tls.client, &self.raw) {
                Start::Waiting => return Ok(None),
                Start::Ready(state) => tls.handshake = Some(*state),
                Start::Alert(_) => return Err(REFUSED),
                Start::Retry => return Err(RETRY),
                Start::Unusable => return Err(FAILED),
            }
        }
        let Some(state) = tls.handshake.as_mut() else {
            return Err(FAILED);
        };
        let end = match state.advance(&self.raw) {
            Progress::Incomplete => return Ok(None),
            Progress::Complete(end) => end,
            Progress::Alert(_) => return Err(REFUSED),
            Progress::Broken => return Err(FAILED),
        };
        let answer =
            state.answer(self.host.as_bytes(), self.rtc, &self.request).map_err(super::refused)?;
        /*
         * The handshake is over: its state, the flight and the private keys
         * are not needed again. Whatever followed the server's Finished in
         * the same read is response.
         */
        tls.handshake = None;
        tls.client.wipe_private();
        tls.app = Some(answer.app);
        self.raw = self.raw.get(end..).map(<[u8]>::to_vec).unwrap_or_default();
        self.request = Vec::new();
        self.out = answer.flight;
        self.sent = 0;
        self.phase = Phase::Send(After::Response);
        self.open_records();
        Ok(None)
    }
}
