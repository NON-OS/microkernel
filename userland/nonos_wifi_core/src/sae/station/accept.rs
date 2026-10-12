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

//! The access point's commit taken, or a frame discarded.

use super::super::commit::confirm;
use super::super::frame::{confirm_body, parse_commit, SEQ_CONFIRM, STATUS_SUCCESS};
use super::{SaeFailure, SaeState, SaeStation, SaeStep};

impl SaeStation {
    pub(super) fn accept_commit(&mut self, body: &[u8]) -> SaeStep {
        let Some(peer) = parse_commit(body) else {
            return self.discard(SaeFailure::BadCommit);
        };
        // A commit equal to this station's own is a reflection: drop it silently.
        if peer.scalar_bytes == self.own_scalar && peer.element_bytes == self.own_element {
            return SaeStep::Ignore;
        }
        let Some(keys) = self.commit.derive_keys(&peer.scalar, &peer.element) else {
            return self.discard(SaeFailure::BadCommit);
        };
        self.keys = keys;
        self.peer_scalar = peer.scalar_bytes;
        self.peer_element = peer.element_bytes;
        self.state = SaeState::Confirmed;
        // The first confirm this station sends carries send-confirm 1.
        let send_confirm = 1u16;
        let c = confirm(
            &self.keys.kck,
            send_confirm,
            &self.own_scalar,
            &self.own_element,
            &self.peer_scalar,
            &self.peer_element,
        );
        SaeStep::Send { seq: SEQ_CONFIRM, status: STATUS_SUCCESS, body: confirm_body(send_confirm, &c) }
    }

    /// Drop a frame that does not authenticate, as 12.4.8.6 says, and keep
    /// why: the exchange runs on until the AP's own confirm or the driver's
    /// deadline.
    pub(super) fn discard(&mut self, why: SaeFailure) -> SaeStep {
        self.failure = Some(why);
        SaeStep::Ignore
    }
}
