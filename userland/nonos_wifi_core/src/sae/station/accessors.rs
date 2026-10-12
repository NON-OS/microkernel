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

//! What an SAE exchange reports: its state, why it failed, the PMK and
//! PMKID once accepted, and the commit frame to send.

use super::*;

impl SaeStation {
    pub fn state(&self) -> SaeState {
        self.state
    }

    pub fn failure(&self) -> Option<SaeFailure> {
        self.failure
    }

    /// The PMK and PMKID once accepted.
    pub fn pmk(&self) -> Option<([u8; 32], [u8; 16])> {
        (self.state == SaeState::Accepted).then_some((self.keys.pmk, self.keys.pmkid))
    }

    /// The commit to send: (transaction sequence, status code, body).
    pub fn commit_frame(&self) -> (u16, u16, Vec<u8>) {
        let status = if self.h2e { STATUS_SAE_HASH_TO_ELEMENT } else { STATUS_SUCCESS };
        let body = commit_body(
            &self.own_scalar,
            &self.own_element,
            &self.token[..self.token_len],
            self.h2e,
        );
        (SEQ_COMMIT, status, body)
    }
}
