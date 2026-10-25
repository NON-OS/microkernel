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

//! The peer's confirm ends the exchange once it verifies; one that does not
//! is discarded, and the exchange runs on.

use super::*;

impl SaeStation {
    pub(super) fn on_confirm(&mut self, status: u16, body: &[u8]) -> SaeStep {
        if status != STATUS_SUCCESS {
            return self.discard(SaeFailure::Rejected(status));
        }
        if self.state != SaeState::Confirmed {
            return SaeStep::Ignore;
        }
        let Some((peer_sc, got)) = parse_confirm(body) else {
            return self.discard(SaeFailure::BadConfirm);
        };
        let want = confirm(
            &self.keys.kck,
            peer_sc,
            &self.peer_scalar,
            &self.peer_element,
            &self.own_scalar,
            &self.own_element,
        );
        let mut diff = 0u8;
        for (a, b) in got.iter().zip(want.iter()) {
            diff |= a ^ b;
        }
        if diff != 0 {
            return self.discard(SaeFailure::BadConfirm);
        }
        self.state = SaeState::Accepted;
        self.failure = None;
        SaeStep::Accepted { pmk: self.keys.pmk }
    }

    /// Drop a frame that does not authenticate, as 12.4.8.6 says, and keep
    /// why: the exchange runs on until the AP's own confirm or the driver's
    /// deadline.
    pub(super) fn discard(&mut self, why: SaeFailure) -> SaeStep {
        self.failure = Some(why);
        SaeStep::Ignore
    }
}
