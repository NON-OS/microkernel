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

use super::types::Transport;
use crate::constants::LEG_QUEUE_NOTIFY;

impl Transport {
    /// Tell the device queue `queue` has a buffer to fill. virtio-rng has
    /// one queue; the legacy register takes its index, a modern doorbell is
    /// that queue's own and takes the index too.
    pub fn notify(self, queue: u16) {
        match self {
            Self::Legacy(regs) => unsafe { regs.w16(LEG_QUEUE_NOTIFY, queue) },
            Self::Modern(m) => m.notify.w16(m.doorbell, queue),
        }
    }
}
