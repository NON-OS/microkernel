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
use crate::constants::{LEG_ISR, LEG_QUEUE_NOTIFY};

impl Transport {
    /// Tell the device queue `queue` has a request.
    pub fn notify(self, queue: u16) {
        match self {
            Self::Legacy(regs) => unsafe { regs.w16(LEG_QUEUE_NOTIFY, queue) },
            Self::Modern(m) => m.notify.w16(m.doorbell, queue),
        }
    }

    /// Read, and so clear, the interrupt status; on INTx that lowers the
    /// line (see `io::rearm`). With MSI-X the value is unused.
    pub fn isr(self) -> u8 {
        match self {
            Self::Legacy(regs) => unsafe { regs.r8(LEG_ISR) },
            Self::Modern(m) => m.isr.r8(0),
        }
    }
}
