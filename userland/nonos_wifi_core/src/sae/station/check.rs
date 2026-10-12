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

//! The access point's confirm checked.

use super::super::commit::confirm;
use super::super::frame::{parse_confirm, STATUS_SUCCESS};
use super::{SaeFailure, SaeState, SaeStation, SaeStep};

impl SaeStation {
    pub(super) fn on_confirm(&mut self, status: u16, body: &[u8]) -> SaeStep {
        if status != STATUS_SUCCESS {
            return self.fail(SaeFailure::Rejected(status));
        }
        if self.state != SaeState::Confirmed {
            return SaeStep::Ignore;
        }
        let Some((peer_sc, got)) = parse_confirm(body) else {
            return self.fail(SaeFailure::BadConfirm);
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
            return self.fail(SaeFailure::BadConfirm);
        }
        self.state = SaeState::Accepted;
        SaeStep::Accepted
    }
}
