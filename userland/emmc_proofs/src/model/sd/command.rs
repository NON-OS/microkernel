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

//! One command: CMD0 from any state, then the card's identification mode
//! commands, then its data transfer mode ones; the rest go unanswered.

use super::super::card::{Out, Reply, St};
use super::card::out;
use super::SdCard;

impl SdCard {
    pub fn command(&mut self, idx: u8, arg: u32, host_blocks: u16, now: u64) -> Out {
        self.tick(now);
        if matches!(self.state, St::Off | St::Inactive) {
            return out(Reply::Silent);
        }
        // An ACMD is the command after an accepted CMD55; any command ends it.
        let app = std::mem::take(&mut self.app);
        if idx == 0 {
            // GO_IDLE_STATE also puts the bus back to one line (4.2.1).
            (self.state, self.polls, self.width) = (St::Idle, 0, 1);
            return out(Reply::Nothing);
        }
        match self.identification(app, idx, arg) {
            Some(o) => o,
            None => self.transfer(app, idx, arg, host_blocks, now),
        }
    }
}
