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

//! Between requests, the firmware's health: a command still in flight went
//! unanswered, or a transmit queue's oldest frame did. Either way what the
//! firmware held cannot be undone by commands, so it is restarted.

use crate::firmware::gen3::outcome::Failure;

use super::state::{Radio, State};

impl Radio {
    /// Restart the firmware if it stopped answering; whether it did.
    pub(super) fn failed_while_idle(&mut self, now: u64) -> bool {
        let unanswered = matches!(&self.state, State::Up(up) if up.dev.cmdq.busy());
        let why = if unanswered {
            Failure::Unanswered
        } else if self.join.tx_stuck(now) {
            Failure::TxStuck
        } else {
            return false;
        };
        self.firmware_failed(now, why);
        true
    }
}
