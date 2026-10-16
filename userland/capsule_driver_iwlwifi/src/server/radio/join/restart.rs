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

//! The join state after the firmware restarted.

use crate::firmware::gen3::join::inbox::{Inbox, Queues};
use crate::firmware::gen3::txq::{TxQueue, TXQ_STRIDE};

use super::state::Join;

impl Join {
    /// After the firmware stopped: a joined link is lost, and no queue or
    /// command the firmware held is live. The transmit region, the network's
    /// name and the history stay.
    pub(in super::super) fn restart(&mut self) {
        if self.joined.take().is_some() {
            self.history.lost();
        }
        self.q = Queues { mgmt: TxQueue::new(0), data: TxQueue::new(TXQ_STRIDE) };
        self.inbox = Inbox::new();
    }
}
