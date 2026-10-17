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

//! A frame from the access point, and this station's own commit: sent at
//! the start and again with an anti-clogging token.

use alloc::vec::Vec;

use super::super::frame::{
    commit_body, SEQ_COMMIT, SEQ_CONFIRM, STATUS_SAE_HASH_TO_ELEMENT, STATUS_SUCCESS,
};
use super::{SaeState, SaeStation, SaeStep};

impl SaeStation {
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

    /// Feed an SAE Authentication frame from the AP.
    pub fn on_frame(&mut self, seq: u16, status: u16, body: &[u8]) -> SaeStep {
        if self.state == SaeState::Accepted {
            return SaeStep::Ignore;
        }
        match seq {
            SEQ_COMMIT => self.on_commit(status, body),
            SEQ_CONFIRM => self.on_confirm(status, body),
            _ => SaeStep::Ignore,
        }
    }
}
