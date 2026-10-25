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

//! The server's flight judged in one pass, and the request sealed behind
//! the client's Finished only when every check held.

use nonos_tls::{HandshakeState, Progress, Refusal, Start};

use super::said::*;
use super::state::Exchange;

impl Exchange {
    /// Judge the server's flight in one pass, and seal the request behind
    /// the client's Finished only when every check held.
    pub(super) fn check(&mut self) -> Result<(), &'static str> {
        let client = self.flight.as_ref().ok_or(NO_ANSWER)?;
        let host = self.host.as_bytes();
        if self.handshake.is_none() {
            match HandshakeState::begin(client, &self.server) {
                Start::Ready(state) => self.handshake = Some(*state),
                Start::Waiting => return Err(NO_CHAIN),
                Start::Alert(_) => return Err(ALERTED),
                Start::Retry => return Err(RETRY),
                Start::Unusable => return Err(UNUSABLE),
            }
        }
        self.tls.server_hello = true;
        let state = self.handshake.as_mut().ok_or(NO_ANSWER)?;
        let end = match state.advance(&self.server) {
            Progress::Complete(end) => end,
            Progress::Incomplete => return Err(NO_CHAIN),
            Progress::Alert(_) => return Err(ALERTED),
            Progress::Broken => return Err(BROKEN),
        };
        self.tls.encrypted_record = true;
        self.tls.certificate = true;
        let now = self.now.ok_or(NO_CLOCK)?;
        let http = crate::wallet::rpc::http_post(host, &self.body);
        let answer = match state.answer(host, now, &http) {
            Ok(answer) => answer,
            Err(Refusal::Unverified) => {
                let problem = state.cert_problem(host, now);
                self.tls.mark_refused(problem);
                return Err(refused_why(problem));
            }
            Err(Refusal::Alert(_)) => return Err(ALERTED),
            Err(Refusal::Incomplete) => return Err(NO_CHAIN),
            Err(Refusal::Seal) => return Err(NO_ANSWER),
            Err(Refusal::TooSlow) => return Err(SLOW),
        };
        self.tls.mark_trusted();
        self.sealed(answer.app, answer.flight, end);
        Ok(())
    }
}
